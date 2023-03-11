#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(dead_code)]

use std::sync::Mutex;

use crate::money_repo::Repo;
pub mod account_display;
pub mod config;
pub mod money_repo;
pub mod reader;
pub mod handlers;

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
            handlers::entries,
            handlers::transactions,
            handlers::add_transaction,
            handlers::update_transaction,
            handlers::delete_transaction,
            handlers::accounts,
            handlers::add_account,
            handlers::update_account,
            handlers::delete_account,
            handlers::schedules,
            handlers::add_schedule,
            handlers::update_schedule,
            handlers::generate,
            handlers::end_date,
            handlers::transaction,
            handlers::update_settings,
            handlers::settings,
            handlers::config,
            handlers::load_csv,
            handlers::load_file,
            handlers::new_file
        ])
        .run(context)
        .expect("error while running tauri application");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
