//use tauri::menu::MenuBuilder;

// pub fn build_menu(context: &tauri::Context<tauri::utils::assets::EmbeddedAssets>) -> Menu {
//     let open_file = CustomMenuItem::new("open".to_string(), "Open");
//     let new_file = CustomMenuItem::new("new".to_string(), "New");
//     let about = CustomMenuItem::new("about".to_string(), "About Tankpipe");
//     let preferences = CustomMenuItem::new("preferences".to_string(), "Preferences");
//     let app_name = &context.package_info().name;
//     let os_menu = tauri::Menu::os_default(app_name);

//     let mut submenus: Vec<Submenu> = vec![];
//     for item in os_menu.items {
//         match item {
//             MenuEntry::Submenu(s) =>  {
//                 if s.title.eq("File") {
//                     submenus.push(Submenu::new("File", Menu::new()
//                         .add_item(new_file.clone())
//                         .add_item(open_file.clone())));
//                 } else if s.title.eq(app_name) {
//                     submenus.push(Submenu::new(app_name.to_string(), Menu::new()
//                         .add_item(about.clone())
//                         .add_native_item(MenuItem::Separator)
//                         .add_item(preferences.clone())
//                         .add_native_item(MenuItem::Separator)
//                         .add_native_item(MenuItem::Quit)));
//                 } else  {
//                     submenus.push(s);
//                 }
//             },
//             _ => (),
//         }
//     }
//     let mut menu = Menu::new();

//     for s in submenus {
//         menu = menu.add_submenu(s);
//     }
//     menu
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
