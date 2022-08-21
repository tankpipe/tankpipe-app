#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused_imports)]
#![allow(dead_code)]

use account_display::{DateParam, NewAccount, NewSchedule};
use accounts::book_repo::load_books;
use accounts::books::Settings;
use accounts::{
    account::{Account, Entry},
    account::{Schedule, Transaction},
    books::Books,
};
use chrono::NaiveDate;
use config::Config;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use uuid::Uuid;

use tauri::State;

use crate::money_repo::Repo;
pub mod account_display;
pub mod config;
pub mod money_repo;

pub struct BooksState(Mutex<Repo>);

fn main() {
    let repo = Repo::load_startup().expect("Unable to initialise app");

    use tauri::Manager;
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .manage(BooksState(Mutex::from(repo)))
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            transactions,
            add_transaction,
            update_transaction,
            accounts,
            add_account,
            update_account,
            schedules,
            add_schedule,
            update_schedule,
            generate,
            end_date,
            transaction,
            update_settings,
            settings,
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn transaction(
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
fn transactions(state: tauri::State<BooksState>, account_id: Uuid) -> Vec<Entry> {
    println!("Fetching transactions for {}", account_id);
    let mutex_guard = state.0.lock().unwrap();
    let x = mutex_guard.books.account_entries(account_id);
    x.unwrap()
}

#[tauri::command]
fn add_transaction(
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
fn update_transaction(
    state: tauri::State<BooksState>,
    transaction: Transaction,
) -> Result<(), String> {
    println!("Updating transaction {:?}", transaction);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_transaction(transaction))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
fn accounts(state: tauri::State<BooksState>) -> Vec<Account> {
    println!("Fetching accounts");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.accounts()
}

#[tauri::command]
fn add_account(state: tauri::State<BooksState>, account: NewAccount) -> Result<(), String> {
    println!("Adding transaction {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    let _ = mutex_guard.books.add_account(account.to_account());
    error_handler(mutex_guard.save())
}

#[tauri::command]
fn update_account(state: tauri::State<BooksState>, account: Account) -> Result<(), String> {
    println!("Adding transaction {}", account.name);
    let mut mutex_guard = state.0.lock().unwrap();
    let _x = mutex_guard.books.add_account(account);
    error_handler(mutex_guard.save())
}

#[tauri::command]
fn schedules(state: tauri::State<BooksState>) -> Vec<Schedule> {
    println!("Fetching scheduled transactions");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.schedules().to_vec()
}

#[tauri::command]
fn add_schedule(state: tauri::State<BooksState>, schedule: NewSchedule) -> Result<(), String> {
    println!("Adding transaction {}", schedule.description);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_schedule(schedule.to_schedule()))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
fn update_schedule(state: tauri::State<BooksState>, schedule: Schedule) -> Result<(), String> {
    println!("Adding schedule {}", schedule.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_schedule(schedule))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
fn end_date(state: tauri::State<BooksState>) -> Option<DateParam> {
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
fn generate(state: tauri::State<BooksState>, date: DateParam) {
    println!("Generating to {}", date.date);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.generate(date.date);
}

#[tauri::command]
fn update_settings(state: tauri::State<BooksState>, settings: Settings) -> Result<(), String> {
    println!("Updating settings: {:?}", settings);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.settings = settings;
    error_handler(mutex_guard.save())
}

#[tauri::command]
fn settings(state: tauri::State<BooksState>) -> Settings {
    println!("Fetching settings");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.settings.clone()
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
