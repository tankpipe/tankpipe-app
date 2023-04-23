#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(dead_code)]

use std::sync::Mutex;
use tauri::{Menu, CustomMenuItem, Submenu, MenuEntry};
use crate::handlers::{account, transaction, schedule, repo};
use crate::money_repo::Repo;

pub mod account_display;
pub mod config;
pub mod money_repo;
pub mod reader;
mod handlers;

pub struct BooksState(Mutex<Repo>);

#[derive(Clone, serde::Serialize)]
struct Payload {}

fn main() {
    let repo = Repo::load_startup().expect("Unable to initialise app");
    use tauri::Manager;
    let context = tauri::generate_context!();
    let menu = build_menu(&context);

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
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
              "open" => emit_event(&event, "file-open"),
              "new" => emit_event(&event, "file-new"),
              _ => {}
            }
          })
        .invoke_handler(tauri::generate_handler![
            transaction::entries,
            transaction::transactions,
            transaction::add_transaction,
            transaction::update_transaction,
            transaction::delete_transaction,
            transaction::transaction,
            account::accounts,
            account::add_account,
            account::update_account,
            account::delete_account,
            schedule::schedules,
            schedule::add_schedule,
            schedule::update_schedule,
            schedule::generate,
            schedule::end_date,
            repo::update_settings,
            repo::settings,
            repo::config,
            repo::update_config,
            repo::load_csv,
            repo::load_file,
            repo::new_file
        ])
        .run(context)
        .expect("error while running tauri application");
}

fn emit_event(event: &tauri::WindowMenuEvent, event_name: &str) {
    match event.window().emit(event_name, Payload {}) {
        Ok(_) => {},
        Err(e) => println!("Error on {} event: {}", event_name, e),
    }
}

fn build_menu(context: &tauri::Context<tauri::utils::assets::EmbeddedAssets>) -> Menu {
    let open_file = CustomMenuItem::new("open".to_string(), "Open");
    let new_file = CustomMenuItem::new("new".to_string(), "New");
    let os_menu = tauri::Menu::os_default(&context.package_info().name);

    let mut submenus: Vec<Submenu> = vec![];
    for item in os_menu.items {
        match item {
            MenuEntry::Submenu(s) =>  {
                if s.title.eq("File") {
                    submenus.push(Submenu::new("File", Menu::new().add_item(new_file.clone()).add_item(open_file.clone())));
                } else  {
                    submenus.push(s);
                }
            },
            _ => (),
        }
    }
    let mut menu = Menu::new();

    for s in submenus {
        menu = menu.add_submenu(s);
    }
    menu
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
