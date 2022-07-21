#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#![allow(unused_imports)]
#![allow(dead_code)]

use accounts::{books::Books, book_repo, account::Transaction, account::Account};
use account_display::{NewTransaction, NewAccount};
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
    .invoke_handler(tauri::generate_handler![greet, transactions, accounts, add_transaction, add_account, update_account, update_transaction])
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
fn accounts(state: tauri::State<BooksState>) -> Vec<Account> {
  println!("Fetching accounts");
  let mutex_guard = state.0.lock().unwrap();
  mutex_guard.accounts()  
}

#[tauri::command]
fn add_transaction(state: tauri::State<BooksState>,transaction: NewTransaction)  {
  println!("Adding transaction {}", transaction.description);
  let mut mutex_guard = state.0.lock().unwrap();
  let x = mutex_guard.add_transaction(transaction.to_transaction());
  match x {
    Ok(_) => x.unwrap(),
    Err(e) => println!("{}", e.error),
  }
}

#[tauri::command]
fn update_transaction(state: tauri::State<BooksState>,transaction: Transaction)  {
  println!("Adding transaction {}", transaction.description);
  let mut mutex_guard = state.0.lock().unwrap();
  let x = mutex_guard.update_transaction(transaction);
  match x {
    Ok(_) => x.unwrap(),
    Err(e) => println!("{}", e.error),
  }
}

#[tauri::command]
fn add_account(state: tauri::State<BooksState>, account: NewAccount)  {
  println!("Adding transaction {}", account.name);
  let mut mutex_guard = state.0.lock().unwrap();
  let x = mutex_guard.add_account(account.to_account());  
}

#[tauri::command]
fn update_account(state: tauri::State<BooksState>, account: Account)  {
  println!("Adding transaction {}", account.name);
  let mut mutex_guard = state.0.lock().unwrap();
  let x = mutex_guard.add_account(account);  
}



// #[tauri::command]
// fn load_books() -> crate::book::Books {
//   crate::book_repo::load_books("/Users/connecting/dev/rust/money/src-tauri/src/books.json").unwrap()
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}