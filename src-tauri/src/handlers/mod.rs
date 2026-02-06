pub mod account;
pub mod transaction;
pub mod schedule;
pub mod modifier;
pub mod repo;

pub fn error_handler(x: Result<(), accounts::books::BooksError>) -> Result<(), String> {
    match x {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e.error);
            Err(e.error)
        }
    }
}