use accounts::schedule::{Modifier};
use uuid::Uuid;
use crate::{BooksState, handlers::error_handler};

#[tauri::command]
pub fn modifiers(state: tauri::State<BooksState>) -> Vec<Modifier> {
    println!("Fetching modifiers");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.modifiers().to_vec().into_iter().cloned().collect()
}

#[tauri::command]
pub fn get_modifier(state: tauri::State<BooksState>, modifier_id: Uuid) -> Result<Modifier, String> {
    println!("Fetching modifier {}", modifier_id);
    let mutex_guard = state.0.lock().unwrap();
    match mutex_guard.books.get_modifier(modifier_id) {
        Ok(modifier) => Ok(modifier),
        Err(e) => Err(format!("Modifier {} not found: {}", modifier_id, e.error)),
    }
}

#[tauri::command]
pub fn add_modifier(state: tauri::State<BooksState>, mut modifier: Modifier) -> Result<(), String> {
    println!("Adding modifier: {}", modifier.name);
    modifier.id = Uuid::new_v4();
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_modifier(modifier))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_modifier(state: tauri::State<BooksState>, modifier: Modifier) -> Result<(), String> {
    println!("Updating modifier {}", modifier.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_modifier(modifier))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn delete_modifier(state: tauri::State<BooksState>, id: Uuid) -> Result<(), String> {
    println!("Deleting modifier {}", id);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.delete_modifier(&id))?;
    error_handler(mutex_guard.save())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
