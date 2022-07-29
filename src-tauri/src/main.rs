#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#![allow(unused_imports)]
#![allow(dead_code)]

use accounts::{books::Books, book_repo, account::{Transaction, Schedule}, account::Account};
use account_display::{NewTransaction, NewAccount, NewSchedule, DateParam};
use chrono::NaiveDate;
use uuid::Uuid;
use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
  }, str::FromStr,
};

use tauri::State;
pub mod account_display;


pub struct BooksState(Mutex<Books>);

fn main() {
  let books = book_repo::load_books("/Users/connecting/dev/rust/money/src-tauri/src/books.json").unwrap();
  
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
    .manage(BooksState(Mutex::from(books)))
    .menu(if cfg!(target_os = "macos") {
      tauri::Menu::os_default(&context.package_info().name)
    } else {
      tauri::Menu::default()
    })
    .invoke_handler(tauri::generate_handler![
        transactions, add_transaction, update_transaction, 
        accounts, add_account, update_account, 
        schedules, add_schedule, update_schedule,
        generate, end_date
    ])
    .run(context)
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn transactions(state: tauri::State<BooksState>, account_id: Uuid) -> Vec<Transaction> {
  println!("Fetching transactions for {}", account_id);
  let mutex_guard = state.0.lock().unwrap();
  let x = mutex_guard.account_transactions(account_id);
  x.unwrap()
}

#[tauri::command]
fn add_transaction(state: tauri::State<BooksState>,transaction: NewTransaction) -> Result<(), String> {
  println!("Adding transaction {}", transaction.description);
  let mut mutex_guard = state.0.lock().unwrap();
  error_handler(mutex_guard.add_transaction(transaction.to_transaction()))
}

#[tauri::command]
fn update_transaction(state: tauri::State<BooksState>,transaction: Transaction) -> Result<(), String> {
  println!("Adding transaction {}", transaction.description);
  let mut mutex_guard = state.0.lock().unwrap();
  error_handler(mutex_guard.update_transaction(transaction))
}

#[tauri::command]
fn accounts(state: tauri::State<BooksState>) -> Vec<Account> {
  println!("Fetching accounts");
  let mutex_guard = state.0.lock().unwrap();
  mutex_guard.accounts()  
}

#[tauri::command]
fn add_account(state: tauri::State<BooksState>, account: NewAccount)  {
  println!("Adding transaction {}", account.name);
  let mut mutex_guard = state.0.lock().unwrap();
  let _x = mutex_guard.add_account(account.to_account());  
}

#[tauri::command]
fn update_account(state: tauri::State<BooksState>, account: Account)  {
  println!("Adding transaction {}", account.name);
  let mut mutex_guard = state.0.lock().unwrap();
  let _x = mutex_guard.add_account(account);  
}

#[tauri::command]
fn schedules(state: tauri::State<BooksState>) -> Vec<Schedule> {
  println!("Fetching scheduled transactions");
  let mutex_guard = state.0.lock().unwrap();
  mutex_guard.schedules().to_vec()
}


#[tauri::command]
fn add_schedule(state: tauri::State<BooksState>, schedule: NewSchedule) -> Result<(), String> {
  println!("Adding transaction {}", schedule.description);
  let mut mutex_guard = state.0.lock().unwrap();
  error_handler(mutex_guard.add_schedule(schedule.to_schedule()))
}

#[tauri::command]
fn update_schedule(state: tauri::State<BooksState>, schedule: Schedule) -> Result<(), String> {
  println!("Adding schedule {}", schedule.name);
  let mut mutex_guard = state.0.lock().unwrap();
  error_handler(mutex_guard.update_schedule(schedule))
}

#[tauri::command]
fn end_date(state: tauri::State<BooksState>) -> Option<DateParam> {
  let mutext_guard = state.0.lock().unwrap();
  
  match mutext_guard.end_date() {
    Some(d) => {
      println!("{}", d);
      Some(DateParam{date: d})      
  },
    None => None,
}
  
}

#[tauri::command]
fn generate(state: tauri::State<BooksState>, date: DateParam) {
  println!("Generating to {}", date.date);
  let mut mutex_guard = state.0.lock().unwrap();
   mutex_guard.generate(date.date);   
}

// #[tauri::command]
// fn load_books() -> crate::book::Books {
//   crate::book_repo::load_books("/Users/connecting/dev/rust/money/src-tauri/src/books.json").unwrap()
// }

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