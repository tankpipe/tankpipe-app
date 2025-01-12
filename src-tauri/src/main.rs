#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(dead_code)]

use std::error::Error;
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{App, Emitter, Manager};
use tauri_plugin_aptabase::EventTracker;
use crate::handlers::{account, transaction, schedule, repo};
use crate::money_repo::Repo;
//use crate::account_display::Analytics;
//use tauri_plugin_aptabase::EventTracker;
//use data_encoding::BASE64;

pub mod account_display;
pub mod config;
pub mod about;
pub mod money_repo;
pub mod reader;
mod handlers;
mod menu;

const ANALYTICS: &str = "QS1FVS0xMzc4MTM4OTE0";
pub struct BooksState(Mutex<Repo>);

#[derive(Clone, serde::Serialize)]
struct Payload {}


fn main() {
    let repo = Repo::load_startup().expect("Unable to initialise app");
    //let input: Vec<u8> = ANALYTICS.into();
    //let binding = BASE64.decode(&input).unwrap();
    //let s = String::from_utf8_lossy(&binding);
    #[cfg(not(debug_assertions))]
    let analytics = Analytics::from_repo(&repo);


    tauri::Builder::default()
        //.plugin(tauri_plugin_aptabase::Builder::new(&s).build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            #[cfg(not(debug_assertions))]
            {
                app.track_event("app_started", Some(::serde_json::json!(analytics.clone())));
            }
            build_menus(app)?;
            Ok(())
        })
        .manage(BooksState(Mutex::from(repo)))
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
            repo::about,
            repo::update_config,
            repo::load_csv,
            repo::load_file,
            repo::new_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn build_menus(app: &mut App) -> Result<(), Box<dyn Error>> {

    let about = MenuItemBuilder::new("About...")
        .id("about")
        .build(app)?;

    let preferences = MenuItemBuilder::new("Settings...")
        .id("preferences")
        .build(app)?;

    let app_submenu = SubmenuBuilder::new(app, "App")
        .item(&about)
        .separator()
        .item(&preferences)
        .separator()
        .quit()
        .build()?;

    let file_open = MenuItemBuilder::new("Open...")
        .id("file-open")
        .accelerator("CmdOrCtrl+O")
        .build(app)?;

    let file_new = MenuItemBuilder::new("New...")
        .id("file-new")
        .accelerator("CmdOrCtrl+N")
        .build(app)?;

    let file_submenu = SubmenuBuilder::new(app, "File")
        .item(&file_new)
        .item(&file_open)
        .build()?;

    let edit_submenu = SubmenuBuilder::new(app, "Edit")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .build()?;

    let menu = MenuBuilder::new(app)
        .items(&[
            &app_submenu,
            &file_submenu,
            &edit_submenu,
            // ... include references to any other submenus
        ])
        .build()?;

    app.set_menu(menu)?;
    app.on_menu_event(move |app, event| {
        match app.emit(event.id().as_ref(), Payload {}) {
            Ok(_) => {},
            Err(e) => println!("Error on {} event: {}", event.id().as_ref(), e),
        }
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
