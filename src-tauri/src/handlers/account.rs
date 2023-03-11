use accounts::account::Account;
use crate::BooksState;
use crate::account_display::NewAccount;
use crate::handlers::error_handler;

#[tauri::command]
pub fn accounts(state: tauri::State<BooksState>) -> Vec<Account> {
    println!("Fetching accounts");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.accounts()
}

#[tauri::command]
pub fn add_account(state: tauri::State<BooksState>, account: NewAccount) -> Result<(), String> {
    println!("Adding account {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    let _ = mutex_guard.books.add_account(account.to_account());
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_account(state: tauri::State<BooksState>, account: Account) -> Result<(), String> {
    println!("Updating account {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    let _x = mutex_guard.books.add_account(account);
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn delete_account(state: tauri::State<BooksState>, account: Account) -> Result<(), String> {
    println!("Deleting account {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.delete_account(&account.id))?;
    error_handler(mutex_guard.save())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
