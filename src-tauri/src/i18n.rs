#[macro_export]
macro_rules! money_error {
    ($($tt:tt)*) => {
        accounts::books::BooksError {
            error: rust_i18n::t!($($tt)*).to_string(),
        }
    };
}
