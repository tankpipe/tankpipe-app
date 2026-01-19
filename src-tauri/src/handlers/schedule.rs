use accounts::account::{Schedule, Transaction, TransactionStatus};
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
pub fn get_schedule(state: tauri::State<BooksState>, schedule_id: Uuid) -> Result<Schedule, String> {
    println!("Fetching schedule {}", schedule_id);
    let mutex_guard = state.0.lock().unwrap();
    match mutex_guard.books.get_schedule(schedule_id) {
        Ok(schedule) => Ok(schedule),
        Err(e) => Err(format!("Schedule {} not found: {}", schedule_id, e.error)),
    }
}

#[tauri::command]
pub fn add_schedule(state: tauri::State<BooksState>, mut schedule: Schedule) -> Result<(), String> {
    println!("Adding schedule: {}", schedule.name);
    schedule.id = Uuid::new_v4();
    for e in schedule.entries.as_mut_slice() {
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
pub fn delete_schedule(state: tauri::State<BooksState>, id: Uuid) -> Result<(), String> {
    println!("Deleting schedule {}", id);
    let mut mutex_guard = state.0.lock().unwrap();
    error_handler(mutex_guard.books.delete_schedule(&id))?;
    error_handler(mutex_guard.save())
}

#[tauri::command]
pub fn schedule_transactions(
    state: tauri::State<BooksState>,
    schedule_id: Uuid,
    status: Option<TransactionStatus>,
) -> Vec<Transaction> {
    println!("Fetching transactions for schedule {} with status filter: {:?}", schedule_id, status);
    let mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.transactions_by_schedule(schedule_id, status)
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

#[tauri::command(async)]
pub fn generate(state: tauri::State<BooksState>, date: DateParam) -> Result<(), String> {
    println!("Generating to {}", date.date);
    let mut mutex_guard = state.0.lock().unwrap();
    mutex_guard.books.generate(date.date);
    error_handler(mutex_guard.save())
}

#[tauri::command(async)]
pub fn generate_by_schedule(state: tauri::State<BooksState>, date: DateParam, schedule_id: Uuid) -> Result<Vec<Transaction>, String> {
    println!("Generating transactions for schedule {} to {}", schedule_id, date.date);
    let mut mutex_guard = state.0.lock().unwrap();
    let transactions = mutex_guard.books.generate_by_schedule(date.date, schedule_id);
    error_handler(mutex_guard.save())?;
    Ok(transactions)
}

#[tauri::command]
pub fn reset_schedule_last_date(state: tauri::State<BooksState>, schedule_id: Uuid) -> Result<Option<DateParam>, String> {
    println!("Resetting last date for schedule {}", schedule_id);
    let mut mutex_guard = state.0.lock().unwrap();
    let result = mutex_guard.books.reset_schedule_last_date(schedule_id);
    error_handler(mutex_guard.save())?;
    Ok(result.map(|date| DateParam { date }))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
