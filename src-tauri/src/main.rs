#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#![allow(dead_code)]

use std::sync::Mutex;
//use crate::account_display::Analytics;
use crate::handlers::{account, transaction, schedule, repo};
//use crate::menu::build_menu;
//use tauri::menu::MenuBuilder;
use crate::money_repo::Repo;
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
    use tauri::Manager;
    let context = tauri::generate_context!();
    //let menu = build_menu(&context);

    //let input: Vec<u8> = ANALYTICS.into();
    //let binding = BASE64.decode(&input).unwrap();
    //let s = String::from_utf8_lossy(&binding);
    #[cfg(not(debug_assertions))]
    let analytics = Analytics::from_repo(&repo);


    tauri::Builder::default()
        //.plugin(tauri_plugin_aptabase::Builder::new(&s).build())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            #[cfg(not(debug_assertions))]
            {
                app.track_event("app_started", Some(::serde_json::json!(analytics.clone())));
            }

            Ok(())
        })
        .manage(BooksState(Mutex::from(repo)))
        //.menu(menu)
        // .on_menu_event(|event| {
        //     match event.menu_item_id() {
        //       "open" => emit_event(&event, "file-open"),
        //       "new" => emit_event(&event, "file-new"),
        //       "about" => emit_event(&event, "about"),
        //       "preferences" => emit_event(&event, "preferences"),
        //       _ => {}
        //     }
        //   })
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
        .run(context)
        .expect("error while running tauri application");
}

// fn emit_event(event: &tauri::WindowMenuEvent, event_name: &str) {
//     match event.window().emit(event_name, Payload {}) {
//         Ok(_) => {},
//         Err(e) => println!("Error on {} event: {}", event_name, e),
//     }
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
