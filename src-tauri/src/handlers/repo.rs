use crate::about::About;
use crate::account_display::ConfigSettings;
use crate::config::Config;
use crate::csv_check::{CsvCheck, CsvMapping};
use crate::handlers::error_handler;
use crate::money_repo::{save_additional_data, Repo};
use crate::reader::{
    check_csv_format, check_date_format, check_sign_reversal, read_headers, read_rows,
    read_transactions, ColumnType, ColumnTypes,
};
use crate::BooksState;
use accounts::account::Account;
use accounts::books::{Settings, MAX_PROJECTION_MONTHS};
use accounts::books_repo::{export_accounts_to_csv, export_to_csv};
use accounts::reconcile::ReconciliationItem;
use std::collections::HashSet;
use std::ffi::OsString;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
pub fn update_settings(state: tauri::State<BooksState>, settings: Settings) -> Result<Settings, String> {
    println!("Updating settings: {:?}", settings);
    let mut mutex_guard = state.0.lock().unwrap();
    let mut settings = settings;
    settings.projection_months = settings.projection_months.min(MAX_PROJECTION_MONTHS);
    mutex_guard.books.settings = settings;
    error_handler(mutex_guard.books.run_checks_and_update())?;
    error_handler(mutex_guard.save())?;
    Ok(mutex_guard.books.settings.clone())
}

#[tauri::command]
pub fn settings(state: tauri::State<BooksState>) -> Settings {
    println!("Fetching settings");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.settings.clone()
}

#[tauri::command]
pub fn config(state: tauri::State<BooksState>) -> Config {
    println!("Fetching config");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.config.clone()
}

#[tauri::command]
pub async fn initialise(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("initialise");
    if app_handle.try_state::<BooksState>().is_some() {
        println!("initialise skipped: BooksState already loaded");
        return Ok(());
    }

    let repo = Repo::load_startup();
    match repo {
        Ok(repo) => {
            let state = BooksState(Mutex::from(repo));
            app_handle.manage(state);
            Ok(())
        }
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub async fn load_with_path(app_handle: tauri::AppHandle, path: String) -> Result<(), String> {
    println!("load_with_path: {:?}", &path);
    let repo = Repo::load_file_and_config(&OsString::from(path));
    match repo {
        Ok(repo) => {
            let state = BooksState(Mutex::from(repo));
            app_handle.manage(state);
            Ok(())
        }
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub async fn load_config() -> Result<Config, String> {
    println!("load_config");
    Repo::load_config().map_err(|e| e.error)
}

#[tauri::command]
pub fn about() -> About {
    About::new()
}

#[tauri::command]
pub fn list_backups(state: tauri::State<BooksState>) -> Result<Vec<String>, String> {
    let mutex_guard = state.0.lock().unwrap();
    let backups = mutex_guard.list_backups().map_err(|e| e.error)?;
    Ok(backups
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}

#[tauri::command]
pub fn restore_backup(state: tauri::State<BooksState>, backup_path: String, run_checks: bool) -> Result<(), String> {
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.restore_backup(&OsString::from(backup_path), run_checks).map_err(|e| e.error)
}

#[tauri::command]
pub fn update_config(
    state: tauri::State<BooksState>,
    config_settings: ConfigSettings,
) -> Result<(), String> {
    println!("Updating config: {:?}", config_settings);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.config.display_date_format = config_settings.display_date_format;
    mutex_guard.config.theme = config_settings.theme;
    error_handler(mutex_guard.save_config())
}

#[tauri::command]
pub fn evaluate_csv(
    state: tauri::State<BooksState>,
    path: String,
    account: Account,
) -> Result<CsvCheck, String> {
    println!("evaluate_csv: {:?}, for account:{:?}", path, account.id);
    let mutex_guard = state.0.lock().unwrap();
    match mutex_guard.additional_data.get_csv_mapping(account.id) {
        Some(mapping) => {
            let csv_check = read_plain_csv(&path, mapping);
            match csv_check {
                Ok(csv_check) => Ok(csv_check.set_mapping_exists(true)),
                Err(e) => Err(e),
            }
        }
        None => check_and_read_plain_csv(&path, account.normal_balance()),
    }
}

fn check_and_read_plain_csv(path: &String, normal_balance: accounts::account::Side) -> Result<CsvCheck, String> {
    match check_csv_format(&path, true) {
        Ok(columns) => {
            let header = read_headers(path).unwrap();
            match read_rows(path, false) {
                Ok(rows) => {
                    let mut date_format: Option<String> = None;
                    if columns.has_column(ColumnType::Date) {
                        if let Some(df) =
                            check_date_format(&rows, columns.index_of(ColumnType::Date))
                        {
                            println!("Date format: {:?}", df);
                            date_format = Some(df);
                        }
                    }
                    let sign_reversal = check_sign_reversal(
                        &rows,
                        &columns,
                        normal_balance,
                        if columns.has_column(ColumnType::Date) {
                            Some(columns.index_of(ColumnType::Date))
                        } else {
                            None
                        },
                        date_format.as_deref(),
                    );

                    let has_header = columns.len() > 0;
                    Ok(CsvCheck::create_new(
                        columns,
                        header,
                        rows,
                        true,
                        date_format,
                        sign_reversal,
                        has_header,
                    ))
                }
                Err(e) => Err(e.error),
            }
        }
        Err(e) => {
            return Err(e.error);
        }
    }
}

fn read_plain_csv(path: &String, csv_mapping: CsvMapping) -> Result<CsvCheck, String> {
    let header = read_headers(path).unwrap();
    match read_rows(path, false) {
        Ok(rows) => Ok(CsvCheck::create_new(
            csv_mapping.columns,
            header,
            rows,
            true,
            csv_mapping.date_format,
            csv_mapping.sign_reversed_columns,
            csv_mapping.has_header,
        )),
        Err(e) => Err(e.error),
    }
}

fn save_csv_mapping(
    repo: &mut Repo,
    account_id: Uuid,
    columns: &[String],
    import_date_format: &str,
    sign_reversed_columns: &HashSet<ColumnType>,
    header_row: bool,
) {
    let current_mapping = repo.additional_data.get_csv_mapping(account_id);
    let mapping_changed = match current_mapping {
        None => true,
        Some(ref existing) => {
            existing.columns.to_vec() != columns
                || existing.date_format.as_deref() != Some(import_date_format)
                || existing.sign_reversed_columns != *sign_reversed_columns
        }
    };

    if mapping_changed {
        repo.additional_data.add_csv_mapping(
            account_id,
            CsvMapping::new(
                columns.to_vec(),
                Some(import_date_format.to_string()),
                sign_reversed_columns.clone(),
                header_row,
            ),
        );
        let _ = save_additional_data(
            &repo.config.current_file.clone().unwrap().path.clone(),
            &repo.additional_data,
        );
    }
}

fn parse_sign_reversed(columns: &[String]) -> HashSet<ColumnType> {
    columns
        .iter()
        .filter_map(|column| ColumnType::from_str(column).ok())
        .filter(|column| *column != ColumnType::Unknown)
        .collect()
}

#[tauri::command]
pub fn import_csv(
    state: tauri::State<BooksState>,
    path: String,
    account_id: Uuid,
    columns: Vec<String>,
    save_mapping: bool,
    has_headers: bool,
    import_date_format: String,
    sign_reversed_columns: Vec<String>,
) -> Result<(), String> {
    println!("import_csv: {:?}, for account:{:?}. columns:{:?} save_mapping:{} has_headers:{} import_date_format:{:?} sign_reversed:{:?}", path, account_id, columns, save_mapping, has_headers, import_date_format, sign_reversed_columns);
    let mut mutex_guard: std::sync::MutexGuard<'_, Repo> = state.0.lock().unwrap();

    // Remove Balance from column types as it is calculated dynamically
    let column_types: Vec<String> = columns
        .into_iter()
        .filter(|c| c != "balance")
        .collect();
    let sign_reversed = parse_sign_reversed(&sign_reversed_columns);

    let account = mutex_guard.books.get_account(&account_id).map_err(|e| e.error)?;

    let load_result = read_transactions(
        &path,
        account_id,
        &import_date_format,
        &ColumnTypes::from_vec(column_types.clone()),
        has_headers,
        account.normal_balance(),
        &sign_reversed,
    );

    match load_result {
        Ok(transactions) => {
            for t in transactions {
                let add_result = mutex_guard.books.add_transaction(t);
                if add_result.is_err() {
                    return Err(add_result.unwrap_err().error);
                }
            }
            error_handler(mutex_guard.save())?;
            if save_mapping {
                save_csv_mapping(
                    &mut mutex_guard,
                    account_id,
                    &column_types,
                    &import_date_format,
                    &sign_reversed,
                    has_headers,
                );
            }
            mutex_guard.check_interest();
            error_handler(mutex_guard.save())?;
            Ok(())
        }
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub fn reconcile_csv(
    state: tauri::State<BooksState>,
    path: String,
    account_id: Uuid,
    columns: Vec<String>,
    save_mapping: bool,
    has_headers: bool,
    import_date_format: String,
    sign_reversed_columns: Vec<String>,
) -> Result<Vec<ReconciliationItem>, String> {
    println!("reconcile_csv_2: {:?}, for account:{:?}. columns:{:?} save_mapping:{} has_headers:{} import_date_format:{:?} sign_reversed:{:?}", path, account_id, columns, save_mapping, has_headers, import_date_format, sign_reversed_columns);
    let mut mutex_guard = state.0.lock().unwrap();
    let sign_reversed = parse_sign_reversed(&sign_reversed_columns);
    let account = mutex_guard.books.get_account(&account_id).map_err(|e| e.error)?;
    let load_result = read_transactions(
        &path,
        account_id,
        &import_date_format,
        &ColumnTypes::from_vec(columns.clone()),
        has_headers,
        account.normal_balance(),
        &sign_reversed,
    );

    match load_result {
        Ok(transactions) => {
            if save_mapping {
                save_csv_mapping(
                    &mut mutex_guard,
                    account_id,
                    &columns,
                    &import_date_format,
                    &sign_reversed,
                    has_headers,
                );
            }
            let reconciliation_result = mutex_guard
                .books
                .prepare_reconciliation(account_id, transactions);
            match reconciliation_result {
                Ok(results) => Ok(results),
                Err(e) => Err(e.error),
            }
        }
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub fn export_csv(
    state: tauri::State<BooksState>,
    path: String,
    account_id: Uuid,
) -> Result<(), String> {
    println!("export_csv: {:?}, for account:{:?}", path, account_id);
    let mutex_guard = state.0.lock().unwrap();
    export_to_csv(&path, &mutex_guard.books, Some(account_id)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_csv_all(state: tauri::State<BooksState>, path: String) -> Result<(), String> {
    println!("export_csv_all: {:?}", path);
    let mutex_guard = state.0.lock().unwrap();
    export_to_csv(&path, &mutex_guard.books, None).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_accounts_csv(state: tauri::State<BooksState>, path: String) -> Result<(), String> {
    println!("export_accounts_csv: {:?}", path);
    let mutex_guard = state.0.lock().unwrap();
    export_accounts_to_csv(&path, &mutex_guard.books).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_file(state: tauri::State<BooksState>, path: String) -> Result<Vec<Account>, String> {
    println!("load_file: {:?}", path);
    let mut mutex_guard = state.0.lock().unwrap();
    let add_result = mutex_guard.load_books(&OsString::from(path));
    if add_result.is_err() {
        let the_error = add_result.unwrap_err().error;
        println!("{}", the_error);
        return Err(the_error);
    }
    Ok(mutex_guard.books.accounts())
}

#[tauri::command]
pub fn load_books(state: tauri::State<BooksState>, path: String) -> Result<Vec<Account>, String> {
    println!("load_books: {:?}", path);
    let mut mutex_guard = state.0.lock().unwrap();
    let add_result = mutex_guard.load_books(&OsString::from(path));
    if add_result.is_err() {
        let the_error = add_result.unwrap_err().error;
        println!("{}", the_error);
        return Err(the_error);
    }
    Ok(mutex_guard.books.accounts())
}

#[tauri::command]
pub fn new_file(state: tauri::State<BooksState>, name: String) -> Result<Vec<Account>, String> {
    println!("new_file {}", name);
    let mut mutex_guard = state.0.lock().unwrap();
    let new_result = mutex_guard.new_books(&name);

    if new_result.is_err() {
        let the_error = new_result.err().unwrap().error;
        println!("{}", the_error);
        return Err(the_error);
    }
    Ok(mutex_guard.books.accounts())
}

#[tauri::command]
pub fn create_first_books(
    app_handle: tauri::AppHandle,
    name: String,
) -> Result<Vec<Account>, String> {
    println!("create_first_books {}", &name);
    let repo = Repo::first_repo(&name).map_err(|e| e.error)?;
    let accounts = repo.books.accounts();
    let state = BooksState(Mutex::from(repo));
    app_handle.manage(state);
    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money_repo::Repo;
    use crate::reader::ColumnTypes;
    use accounts::{
        account::{Account, AccountType},
        reconcile::{ReconciliationItem, ReconciliationMatchStatus},
    };
    use std::collections::HashSet;
    use std::path::Path;

    #[test]
    fn test_reconciliation_between_csv_files() {
        let test_name = format!("Test Books_{}", std::process::id());
        let mut repo = Repo::first_repo(&test_name).unwrap();

        let account = Account::create_new("Test Checking Account", AccountType::Asset);
        let normal_balance = account.normal_balance();
        let account_id = account.id;
        repo.books.add_account(account);

        let manual_transactions = read_transactions(
            &Path::new("test/fixtures/bank_transactions_manual.csv"),
            repo.books.accounts().first().unwrap().id,
            &"%Y-%m-%d", // Default date format
            &ColumnTypes::from_vec(vec![
                "date".to_string(),
                "description".to_string(),
                "amount".to_string(),
                "type".to_string(),
                "category".to_string(),
                "balance".to_string(),
            ]),
            true, // has_headers
            normal_balance,
            &HashSet::new(),
        )
        .unwrap();

        for transaction in manual_transactions {
            repo.books.add_transaction(transaction).unwrap();
        }

        let bank_transactions = read_transactions(
            &Path::new("test/fixtures/bank_transactions.csv"),
            repo.books.accounts().first().unwrap().id,
            &"%Y-%m-%d", // Default date format
            &ColumnTypes::from_vec(vec![
                "date".to_string(),
                "description".to_string(),
                "amount".to_string(),
                "type".to_string(),
                "category".to_string(),
                "balance".to_string(),
            ]),
            true, // has_headers
            normal_balance,
            &HashSet::new(),
        )
        .unwrap();

        let reconciliation_results = repo
            .books
            .prepare_reconciliation(account_id, bank_transactions)
            .unwrap();
        assert!(!reconciliation_results.is_empty());

        let matched_count = reconciliation_results
            .iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::Matched { .. }))
            .count();
        let partial_count = reconciliation_results
            .iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::PartialMatch { .. }))
            .count();
        let mismatch_count = reconciliation_results
            .iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::Mismatch { .. }))
            .count();
        let unmatched_count = reconciliation_results
            .iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::Unmatched))
            .map(|t| {
                println!("{:?}", t);
                true
            })
            .count();
        let reconciliation_count = reconciliation_results
            .iter()
            .filter(|r| matches!(r, ReconciliationItem::Reconciliation { .. }))
            .count();
        let original_count = reconciliation_results
            .iter()
            .filter(|r| matches!(r, ReconciliationItem::Original { .. }))
            .count();

        assert_eq!(14, matched_count, "Should have at least some exact matches");
        assert_eq!(52, partial_count, "Should have some partial matches");
        assert_eq!(30, mismatch_count, "Should have some mismatches");
        assert_eq!(
            1, unmatched_count,
            "Should have some unmatched due to date variations"
        );
        assert_eq!(49, reconciliation_count);
        assert_eq!(48, original_count);
    }
}
