pub mod account;
pub mod interest;
pub mod modifier;
pub mod repo;
pub mod schedule;
pub mod transaction;

pub fn error_handler(x: Result<(), accounts::books::BooksError>) -> Result<(), String> {
    match x {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{}", e.error);
            Err(e.error)
        }
    }
}
