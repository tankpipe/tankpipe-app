use tauri::{Menu, CustomMenuItem, Submenu, MenuEntry};

pub fn build_menu(context: &tauri::Context<tauri::utils::assets::EmbeddedAssets>) -> Menu {
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
