use accounts::account::{Entry, Transaction};
use std::str::FromStr;
use uuid::Uuid;
use crate::{BooksState, handlers::error_handler};


#[tauri::command]
pub fn transaction(
    state: tauri::State<BooksState>,
    transaction_id: Uuid,
) -> Result<Transaction, String> {
    println!("Fetching transaction {}", transaction_id);
    let mutex_guard = state.0.lock().unwrap();
    let x = mutex_guard.books.transaction(transaction_id);
    match x {
        Some(t) => Ok(t),
        None => Err(format!("Transaction {} not found", transaction_id)),
    }
}

#[tauri::command]
pub fn entries(state: tauri::State<BooksState>, account_id: Uuid) -> Result<Vec<Entry>, String> {
    println!("Fetching transactions for {}", account_id);
    let mutex_guard = state.0.lock().unwrap();
    let x = mutex_guard.books.account_entries(account_id);
    match x {
        Ok(_) => Ok(x.unwrap()),
        Err(e) => {
            println!("{}", e.error);
            Err(e.error)
        }
    }

}

#[tauri::command]
pub fn transactions(state: tauri::State<BooksState>, account_id: Uuid) -> Result<Vec<Transaction>, String> {
    println!("Fetching transactions for {}", account_id);
    let mutex_guard = state.0.lock().unwrap();
    let x = mutex_guard.books.account_transactions(account_id);
    match x {
        Ok(_) => Ok(x.unwrap()),
        Err(e) => {
            println!("{}", e.error);
            Err(e.error)
        }
    }

}

#[tauri::command]
pub fn add_transaction(
    state: tauri::State<BooksState>,
    mut transaction: Transaction,
) -> Result<Transaction, String> {
    println!("Adding transaction {:?}", transaction);
    let id = Uuid::new_v4();
    transaction.id = id.clone();
    for e in transaction.entries.as_mut_slice() {
        e.id = Uuid::new_v4();
        e.transaction_id = transaction.id;
    }

    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_transaction(transaction.clone()))?;
    error_handler(mutex_guard.save())?;
    Ok(transaction)
}

#[tauri::command]
pub fn update_transaction(
    state: tauri::State<BooksState>,
    mut transaction: Transaction,
) -> Result<(), String> {
    println!("Updating transaction {:?}", transaction);
    update_transaction_entries(&mut transaction);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_transaction(transaction))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_transactions(
    state: tauri::State<BooksState>,
    mut transactions: Vec<Transaction>,
) -> Result<(), String> {
    println!("Updating transactions {:?}", transactions);

    let mut mutex_guard = state.0.lock().unwrap();
    for transaction in transactions.iter_mut() {
        update_transaction_entries(transaction);
        error_handler(mutex_guard.books.update_transaction(transaction.clone()))?;
    }

    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn delete_transaction(state: tauri::State<BooksState>, id: Uuid) -> Result<(), String> {
    println!("Deleting transaction {}", id);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.delete_transaction(&id))?;
    error_handler(mutex_guard.save())
}

fn update_transaction_entries(transaction: &mut Transaction) {
    for e in transaction.entries.as_mut_slice() {
        let zeros = Uuid::from_str(String::from("00000000-0000-0000-0000-000000000000").as_str()).unwrap();
        if e.id == zeros {
            e.id = Uuid::new_v4();
            e.transaction_id = transaction.id;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
