use accounts::account::Schedule;
use uuid::Uuid;
use crate::{BooksState, handlers::error_handler};
use crate::account_display::DateParam;

#[tauri::command]
pub fn schedules(state: tauri::State<BooksState>) -> Vec<Schedule> {
    println!("Fetching scheduled transactions");
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.schedules().to_vec()
}

#[tauri::command]
pub fn add_schedule(state: tauri::State<BooksState>, mut schedule: Schedule) -> Result<(), String> {
    println!("Adding schedule: {}", schedule.name);
    schedule.id = Uuid::new_v4();
    for mut e in schedule.entries.as_mut_slice() {
        e.schedule_id = schedule.id;
    }
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.add_schedule(schedule))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn update_schedule(state: tauri::State<BooksState>, schedule: Schedule) -> Result<(), String> {
    println!("Updating schedule {}", schedule.name);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.update_schedule(schedule))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn end_date(state: tauri::State<BooksState>) -> Option<DateParam> {
    let mutext_guard = state.0.lock().unwrap();

    match mutext_guard.books.end_date() {
        Some(d) => {
            Some(DateParam { date: d })
        }
        None => None,
    }
}

#[tauri::command]
pub fn generate(state: tauri::State<BooksState>, date: DateParam) -> Result<(), String> {
    println!("Generating to {}", date.date);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.generate(date.date);
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
