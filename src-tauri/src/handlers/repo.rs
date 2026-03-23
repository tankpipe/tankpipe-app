use accounts::books::{Settings};
use accounts::books_repo::{export_accounts_to_csv, export_to_csv};
use accounts::account::Account;
use accounts::reconcile::ReconciliationItem;
use tauri::Manager;
use std::ffi::OsString;
use std::sync::Mutex;
use crate::BooksState;
use crate::about::About;
use crate::account_display::ConfigSettings;
use crate::config::Config;
use crate::handlers::{error_handler};
use crate::money_repo::Repo;
use crate::reader::{check_csv_format, read_headers, read_rows, read_transactions, ColumnTypes};
use crate::csv_check::CsvCheck;
use uuid::Uuid;


#[tauri::command]
pub fn update_settings(state: tauri::State<BooksState>, settings: Settings) -> Result<(), String> {
    println!("Updating settings: {:?}", settings);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.settings = settings;
    error_handler(mutex_guard.save())
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
    let repo =  Repo::load_startup();
    match repo {
        Ok(repo) => {
            let state = BooksState(Mutex::from(repo));  
            app_handle.manage(state);
            Ok(())        
        },
        Err(e) => Err(e.error),
    }
  
}

#[tauri::command]
pub async fn load_with_path(app_handle: tauri::AppHandle, path: String) -> Result<(), String> {
    println!("load_with_path: {:?}", &path);
    let repo =  Repo::load_file_and_config(&OsString::from(path));
    match repo {
        Ok(repo) => {
            let state = BooksState(Mutex::from(repo));  
            app_handle.manage(state);
            Ok(())        
        },
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
    Ok(backups.into_iter().map(|p| p.to_string_lossy().to_string()).collect())
}

#[tauri::command]
pub fn restore_backup(state: tauri::State<BooksState>, backup_path: String) -> Result<(), String> {
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.restore_backup(&OsString::from(backup_path)).map_err(|e| e.error)
}

#[tauri::command]
pub fn update_config(state: tauri::State<BooksState>, config_settings: ConfigSettings) -> Result<(), String>  {
    println!("Updating config: {:?}", config_settings);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.config.display_date_format = config_settings.display_date_format;
    mutex_guard.config.import_date_format = config_settings.import_date_format;
    mutex_guard.config.theme = config_settings.theme;
    error_handler(mutex_guard.save_config())
}

#[tauri::command]
pub fn evaluate_csv(state: tauri::State<BooksState>, path: String, account: Account) -> Result<CsvCheck, String> {
    println!("evaluate_csv: {:?}, for account:{:?}", path, account.id);
    let mutex_guard = state.0.lock().unwrap();
    let mapping = mutex_guard.config.get_csv_mapping(account.id);
    match mapping {
        Some(mapping) => {
            let column_types = ColumnTypes::from_vec(mapping);
            let csv_check = process_csv_with_column_types(&path, column_types);
            match csv_check {
                Ok(csv_check) => Ok(csv_check.set_mapping_exists(true)),
                Err(e) => Err(e),
            }
        }
        None => {
            let result = check_csv_format(&path, true);
            match result {
                Ok(column_types) => process_csv_with_column_types(&path, column_types),
                Err(e) => Err(e.error),
            }
        }
    }
}

fn process_csv_with_column_types(path: &String, column_types: ColumnTypes) -> Result<CsvCheck, String> {
    let header = read_headers(path).unwrap();
    // Read the rows as they are to show as a sample.
    let rows = read_rows(path, false);
    match rows {
        Ok(rows) => Ok(CsvCheck::create_new(column_types, header, rows, true)),
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub fn import_csv(state: tauri::State<BooksState>, path: String, account_id: Uuid, column_types: Vec<String>, save_mapping: bool, has_headers: bool) -> Result<(), String> {
    println!("import_csv: {:?}, for account:{:?}. columns:{:?} save_mapping:{} has_headers:{}", path, account_id, column_types, save_mapping, has_headers);
    let mut mutex_guard = state.0.lock().unwrap();
    
    // Remove Balance from column types as it is calculated dynamically
    let column_types: Vec<String> = column_types.into_iter().filter(|c| c != "balance").collect();
    
    let load_result = read_transactions(&path, account_id, &mutex_guard.config.import_date_format, &ColumnTypes::from_vec(column_types.clone()), has_headers);

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
                let current_mapping = mutex_guard.config.get_csv_mapping(account_id);
                if current_mapping.is_none() || current_mapping.unwrap() != column_types {
                    mutex_guard.config.set_csv_mapping(account_id, column_types);
                    error_handler(mutex_guard.save_config())?;
                }
            }
            mutex_guard.check_interest();
            error_handler(mutex_guard.save())?;
            Ok(())
        },
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub fn reconcile_csv(state: tauri::State<BooksState>, path: String, account_id: Uuid, column_types: Vec<String>, has_headers: bool) -> Result<Vec<ReconciliationItem>, String> {
    println!("reconcile_csv_2: {:?}, for account:{:?}. columns:{:?} has_headers:{}", path, account_id, column_types, has_headers);
    let mutex_guard = state.0.lock().unwrap();
    let load_result = read_transactions(&path, account_id, &mutex_guard.config.import_date_format, &ColumnTypes::from_vec(column_types), has_headers);

    match load_result {
        Ok(transactions) => {
            let reconciliation_result = mutex_guard.books.prepare_reconciliation(account_id, transactions);
            match reconciliation_result {
                Ok(results) => Ok(results),
                Err(e) => Err(e.error),
            }
        },
        Err(e) => Err(e.error),
    }
}

#[tauri::command]
pub fn export_csv(state: tauri::State<BooksState>, path: String, account_id: Uuid) -> Result<(), String> {
    println!("export_csv: {:?}, for account:{:?}", path, account_id);
    let mutex_guard = state.0.lock().unwrap();
    export_to_csv(&path, &mutex_guard.books, Some(account_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_csv_all(state: tauri::State<BooksState>, path: String) -> Result<(), String> {
    println!("export_csv_all: {:?}", path);
    let mutex_guard = state.0.lock().unwrap();
    export_to_csv(&path, &mutex_guard.books, None)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_accounts_csv(state: tauri::State<BooksState>, path: String) -> Result<(), String> {
    println!("export_accounts_csv: {:?}", path);
    let mutex_guard = state.0.lock().unwrap();
    export_accounts_to_csv(&path, &mutex_guard.books)
        .map_err(|e| e.to_string())
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
pub fn create_first_books(app_handle: tauri::AppHandle, name: String) -> Result<Vec<Account>, String> {
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
    use accounts::{account::{Account, AccountType}, reconcile::{ReconciliationItem, ReconciliationMatchStatus}};
    use std::path::Path;

    #[test]
    fn test_reconciliation_between_csv_files() {
        let test_name = format!("Test Books_{}", std::process::id());
        let mut repo = Repo::first_repo(&test_name).unwrap();        
        
        let account = Account::create_new(
            "Test Checking Account",
            AccountType::Asset
        );
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
                "balance".to_string()
            ]),
            true // has_headers
        ).unwrap();

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
                "balance".to_string()
            ]),
            true // has_headers
        ).unwrap();
        
        let reconciliation_results = repo.books.prepare_reconciliation(account_id, bank_transactions).unwrap();
        assert!(!reconciliation_results.is_empty());
        
        let matched_count = reconciliation_results.iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::Matched { .. }))
            .count();
        let partial_count = reconciliation_results.iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::PartialMatch { .. }))
            .count();
        let mismatch_count = reconciliation_results.iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::Mismatch { .. }))
            .count();
        let unmatched_count = reconciliation_results.iter()
            .filter(|r| matches!(r.status(), ReconciliationMatchStatus::Unmatched))
            .count();
        let reconciliation_count = reconciliation_results.iter()
            .filter(|r| matches!(r, ReconciliationItem::Reconciliation { .. }))
            .count();
        let original_count = reconciliation_results.iter()
            .filter(|r| matches!(r, ReconciliationItem::Original { .. }))
            .count();
        
        assert_eq!(6, matched_count, "Should have at least some exact matches");
        assert_eq!(56, partial_count, "Should have some partial matches");
        assert_eq!(30, mismatch_count, "Should have some mismatches");
        assert_eq!(5, unmatched_count, "Should have some unmatched due to date variations");
        assert_eq!(49, reconciliation_count);
        assert_eq!(48, original_count);
    }
}
