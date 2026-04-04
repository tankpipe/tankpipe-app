use std::str::FromStr;

use crate::handlers::error_handler;
use crate::BooksState;
use accounts::interest::Interest;
use uuid::Uuid;

#[tauri::command]
pub fn get_interest(
    state: tauri::State<BooksState>,
    interest_id: Uuid,
) -> Result<Interest, String> {
    println!("Getting interest info for ID {}", interest_id);
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard
        .books
        .get_interest(&interest_id)
        .map_err(|e| e.error)
}

#[tauri::command]
pub fn add_interest(state: tauri::State<BooksState>, mut interest: Interest) -> Result<(), String> {
    println!("Adding interest info for account {:?}", interest);
    interest.id = Uuid::new_v4();
    update_terms(&mut interest);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_interest(interest))?;
    mutex_guard.check_interest();
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_interest(
    state: tauri::State<BooksState>,
    mut interest: Interest,
) -> Result<(), String> {
    println!("Updating interest info {}", interest.id);
    update_terms(&mut interest);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_interest(interest))?;
    mutex_guard.check_interest();
    error_handler(mutex_guard.save())
}

fn update_terms(interest: &mut Interest) {
    let zeros =
        Uuid::from_str(String::from("00000000-0000-0000-0000-000000000000").as_str()).unwrap();
    for t in interest.terms.as_mut_slice() {
        if t.id == zeros {
            t.id = Uuid::new_v4();
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
