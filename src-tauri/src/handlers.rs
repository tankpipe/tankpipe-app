
#![allow(dead_code)]

use accounts::books::Settings;
use accounts::{
    account::{Account, Entry},
    account::{Schedule, Transaction},
};
use std::ffi::OsString;
use std::{
    str::FromStr,
};
use uuid::Uuid;


use crate::BooksState;
use crate::account_display::{NewAccount, DateParam};
use crate::config::Config;
use crate::reader::load_transactions;



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
) -> Result<(), String> {
    println!("Adding transaction {:?}", transaction);
    transaction.id = Uuid::new_v4();
    for mut e in transaction.entries.as_mut_slice() {
        e.id = Uuid::new_v4();
        e.transaction_id = transaction.id;
    }

    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_transaction(transaction))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_transaction(
    state: tauri::State<BooksState>,
    mut transaction: Transaction,
) -> Result<(), String> {
    println!("Updating transaction {:?}", transaction);
    for mut e in transaction.entries.as_mut_slice() {
        let zeros = Uuid::from_str(String::from("00000000-0000-0000-0000-000000000000").as_str()).unwrap();
        if e.id == zeros {
            e.id = Uuid::new_v4();
            e.transaction_id = transaction.id;
        }

    }
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_transaction(transaction))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn delete_transaction(state: tauri::State<BooksState>, id: Uuid) -> Result<(), String> {
    println!("Deleting transaction {}", id);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.delete_transaction(&id))?;
    error_handler(mutex_guard.save())
}


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

#[tauri::command]
pub fn schedules(state: tauri::State<BooksState>) -> Vec<Schedule> {
    println!("Fetching scheduled transactions");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.schedules().to_vec()
}

#[tauri::command]
pub fn add_schedule(state: tauri::State<BooksState>, mut schedule: Schedule) -> Result<(), String> {
    println!("Adding schedule: {}", schedule.name);
    schedule.id = Uuid::new_v4();
    for mut e in schedule.entries.as_mut_slice() {
        e.schedule_id = schedule.id;
    }
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_schedule(schedule))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_schedule(state: tauri::State<BooksState>, schedule: Schedule) -> Result<(), String> {
    println!("Updating schedule {}", schedule.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_schedule(schedule))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn end_date(state: tauri::State<BooksState>) -> Option<DateParam> {
    let mutext_guard = state.0.lock().unwrap();

    match mutext_guard.books.end_date() {
        Some(d) => {
            println!("{}", d);
            Some(DateParam { date: d })
        }
        None => None,
    }
}

#[tauri::command]
pub fn generate(state: tauri::State<BooksState>, date: DateParam) -> Result<(), String> {
    println!("Generating to {}", date.date);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.generate(date.date);
    error_handler(mutex_guard.save())
}

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
pub fn load_csv(state: tauri::State<BooksState>, path: String, account: Account) -> Result<(), String> {
    println!("load_csv: {:?}, for account:{:?}", path, account.id);
    let load_result = load_transactions(path, &account);

    match load_result {
        Ok(transactions) => {
            let mut mutex_guard = state.0.lock().unwrap();
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
    println!("{:?}", add_result);
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

fn error_handler(x: Result<(), accounts::books::BooksError>) -> Result<(), String> {
    match x {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e.error);
            Err(e.error)
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
