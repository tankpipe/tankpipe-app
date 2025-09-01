use accounts::books::Settings;
use accounts::account::Account;
use std::ffi::OsString;
use crate::BooksState;
use crate::about::About;
use crate::account_display::ConfigSettings;
use crate::config::Config;
use crate::handlers::error_handler;
use crate::reader::{check_csv_format, read_headers, read_rows, read_transations_using_header, ColumnTypes};
use crate::csv_check::CsvCheck;


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
pub fn about() -> About {
    About::new()
}

#[tauri::command]
pub fn update_config(state: tauri::State<BooksState>, config_settings: ConfigSettings) -> Result<(), String>  {
    println!("Updating config");
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.config.display_date_format = config_settings.display_date_format;
    mutex_guard.config.import_date_format = config_settings.import_date_format;
    error_handler(mutex_guard.save_config())
}

#[tauri::command]
pub fn evaluate_csv(state: tauri::State<BooksState>, path: String, account: Account) -> Result<CsvCheck, String> {
    println!("evaluate_csv: {:?}, for account:{:?}", path, account.id);
    let _mutex_guard = state.0.lock().unwrap();
    let result = check_csv_format(&path);
    match result {
        Ok(column_types) => {
            let header = read_headers(&path).unwrap();
            let rows = read_rows(&path);
            match rows {
                Ok(rows) => Ok(CsvCheck::create_new(column_types, header, rows)),
                Err(e) => Err(e.error),
            }
        }

        Err(e) => Err(e.error)
    }
}

#[tauri::command]
pub fn import_csv(state: tauri::State<BooksState>, path: String, account: Account, column_types: Vec<String>) -> Result<(), String> {
    println!("import_csv: {:?}, for account:{:?}. columns:{:?}", path, account.id, column_types);
    let mut mutex_guard = state.0.lock().unwrap();
    let load_result = read_transations_using_header(&path, &account, &mutex_guard.config.import_date_format, &ColumnTypes::from_vec(column_types));

    match load_result {
        Ok(transactions) => {
            for t in transactions {
                let add_result = mutex_guard.books.add_transaction(t);
                if add_result.is_err() {
                    return Err(add_result.unwrap_err().error);
                }
            }
            error_handler(mutex_guard.save())
        },
        Err(e) => Err(e.error),
    }
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
