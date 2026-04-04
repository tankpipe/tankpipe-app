use crate::account_display::{DateParam, NewAccount};
use crate::handlers::error_handler;
use crate::BooksState;
use accounts::account::Account;
use uuid::Uuid;

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
    mutex_guard.check_interest();
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_account(state: tauri::State<BooksState>, account: Account) -> Result<(), String> {
    println!("Updating account {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_account(account))?;
    mutex_guard.check_interest();
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn delete_account(state: tauri::State<BooksState>, account: Account) -> Result<(), String> {
    println!("Deleting account {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.delete_account(&account.id))?;
    mutex_guard.check_interest();
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn reconcile_account_transaction(
    state: tauri::State<BooksState>,
    account_id: Uuid,
    transaction_id: Uuid,
) -> Result<(), String> {
    println!(
        "Reconcile account {} transaction {}",
        account_id, transaction_id
    );
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(
        mutex_guard
            .books
            .reconcile_account_transactions(account_id, vec![transaction_id]),
    )?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn reconcile_account_transactions(
    state: tauri::State<BooksState>,
    account_id: Uuid,
    transaction_ids: Vec<Uuid>,
) -> Result<(), String> {
    println!(
        "Reconcile account {} transactions {:?}",
        account_id, transaction_ids
    );
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(
        mutex_guard
            .books
            .reconcile_account_transactions(account_id, transaction_ids),
    )?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn rollback_reconciliation(
    state: tauri::State<BooksState>,
    account_id: Uuid,
    to_date: DateParam,
) -> Result<(), String> {
    println!("Rollback account {} to date {}", account_id, to_date.date);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(
        mutex_guard
            .books
            .rollback_reconciliation(account_id, to_date.date),
    )?;
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
