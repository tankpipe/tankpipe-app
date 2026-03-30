use accounts::account::{Transaction, Entry, TransactionStatus, Side};
use accounts::books::{BooksError};
use csv::StringRecord;
use rust_decimal::prelude::*;
use chrono::{NaiveDate};
use serde::de::Error;
use uuid::Uuid;
use std::ops::Index;
use std::path::{Path};
use rust_decimal_macros::dec;

use serde::{Deserialize, Serialize};
use serde::Deserializer;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
    Unknown,
    Date,
    Description,
    Debit,
    Credit,
    Amount,
    Balance,
}

impl std::str::FromStr for ColumnType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "date" => Ok(ColumnType::Date),
            "description" => Ok(ColumnType::Description),
            "debit" => Ok(ColumnType::Debit),
            "credit" => Ok(ColumnType::Credit),
            "amount" => Ok(ColumnType::Amount),
            "balance" => Ok(ColumnType::Balance),
            _ => Ok(ColumnType::Unknown),
        }
    }

}
impl std::fmt::Display for ColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColumnType::Date => write!(f, "Date"),
            ColumnType::Description => write!(f, "Description"),
            ColumnType::Debit => write!(f, "Debit"),
            ColumnType::Credit => write!(f, "Credit"),
            ColumnType::Amount => write!(f, "Amount"),
            ColumnType::Balance => write!(f, "Balance"),
            _ => write!(f, "Unknown"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnTypes {
    columns: Vec<ColumnType>
}

impl ColumnTypes {
    pub fn index_of(&self, column: ColumnType) -> usize {
        self.columns.iter().position(|c| c == &column).unwrap_or(ColumnType::Unknown as usize)
    }

    pub fn has_column(&self, column: ColumnType) -> bool {
        self.columns.iter().any(|c| c == &column)
    }

    pub fn has_exactly_one_of(&self, column: ColumnType) -> bool {
        self.columns.iter().filter(|&c| c == &column).count() == 1
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn num_known_columns(&self) -> usize {
        self.columns.iter().filter(|c| **c != ColumnType::Unknown).count()
    }

    pub fn from_vec(v: Vec<String>) -> ColumnTypes {
        ColumnTypes{columns: v.iter().map(|c| ColumnType::from_str(c).unwrap()).collect()}
    }
}

impl Index<usize> for ColumnTypes {
    type Output = ColumnType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}

pub fn check_csv_format<P: AsRef<Path>>(path: &P, reverse_dr_cr: bool) -> Result<ColumnTypes, BooksError> {
    let columns = read_columns(path, reverse_dr_cr)?;
    //validate_columns(&columns)?;
    Ok(columns)
}

pub fn check_date_format(rows: &Vec<Vec<String>>, date_column: usize) -> Option<String> {
    fn extract_date_token(cell: &str) -> Option<&str> {
        let trimmed = cell.trim();
        if trimmed.is_empty() {
            return None;
        }

        // Common bank exports append a time portion; keep the date token only.
        let before_space = trimmed.split_whitespace().next().unwrap_or(trimmed);
        let before_t = before_space.split('T').next().unwrap_or(before_space);
        if before_t.is_empty() { None } else { Some(before_t) }
    }

    fn is_date_like(token: &str) -> bool {
        // Only allow digits and a single repeated separator (e.g. 1/2/2024, 2024-02-01).
        let mut separator: Option<char> = None;
        let mut separator_count = 0usize;

        for ch in token.chars() {
            if ch.is_ascii_digit() {
                continue;
            }
            if matches!(ch, '/' | '-' | '.') {
                match separator {
                    None => separator = Some(ch),
                    Some(sep) if sep == ch => {}
                    Some(_) => return false, // mixed separators
                }
                separator_count += 1;
                continue;
            }
            return false;
        }

        if separator_count != 2 {
            return false;
        }

        let Some(sep) = separator else { return false };
        let parts: Vec<&str> = token.split(sep).collect();
        if parts.len() != 3 {
            return false;
        }

        // Require plausible part widths; year commonly 2 or 4 digits.
        parts.iter().all(|p| {
            !p.is_empty()
                && p.len() <= 4
                && p.chars().all(|c| c.is_ascii_digit())
        }) && (parts[0].len() == 4 || parts[2].len() == 4 || parts[2].len() == 2)
    }

    const CANDIDATE_FORMATS: [&str; 18] = [
        "%d/%m/%Y",
        "%m/%d/%Y",
        "%Y/%m/%d",
        "%d-%m-%Y",
        "%m-%d-%Y",
        "%Y-%m-%d",
        "%d.%m.%Y",
        "%m.%d.%Y",
        "%Y.%m.%d",
        "%d/%m/%y",
        "%m/%d/%y",
        "%y/%m/%d",
        "%d-%m-%y",
        "%m-%d-%y",
        "%y-%m-%d",
        "%d.%m.%y",
        "%m.%d.%y",
        "%y.%m.%d",
    ];

    let mut possible: std::collections::HashSet<&'static str> =
        CANDIDATE_FORMATS.into_iter().collect();

    let mut considered = 0usize;
    for row in rows {
        let Some(cell) = row.get(date_column) else { continue };
        let Some(token) = extract_date_token(cell) else { continue };
        if !is_date_like(token) {
            continue;
        }

        considered += 1;
        let mut parsable: std::collections::HashSet<&'static str> = std::collections::HashSet::new();
        for fmt in CANDIDATE_FORMATS {
            if NaiveDate::parse_from_str(token, fmt).is_ok() {
                parsable.insert(fmt);
            }
        }

        if parsable.is_empty() {
            return None;
        }

        possible.retain(|fmt| parsable.contains(fmt));
        if possible.is_empty() {
            return None;
        }

        // Limit work on huge CSVs (we only need a representative sample).
        if considered >= 200 {
            break;
        }
    }

    if considered == 0 || possible.len() != 1 {
        return None;
    }

    possible.iter().next().map(|fmt| fmt.to_string())
}

pub fn read_rows<P: AsRef<Path>>(path: &P, reverse_dr_cr: bool) -> Result<Vec<Vec<String>>, BooksError> {
    let columns = read_columns(path, reverse_dr_cr)?;
    let _ = validate_columns(&columns);
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let mut rows: Vec<Vec<String>> = Vec::new();

            for result in reader.deserialize() {
                match result {
                    Ok(item) => rows.push(item),
                    Err(e) => println!("Skipping row as unabled to process. Error: {:?}", e)
                }
            }
            Ok(rows)
        },
        Err(e) => {
            return Err(crate::books_error!("errors.read_csv_failure", error => e))
        }
     }
}

pub fn read_transactions <P: AsRef<Path>>(path: &P, account_id: Uuid, fmt: &str, columns: &ColumnTypes, has_headers: bool) -> Result<Vec<Transaction>, BooksError> {
    println!("read_transactions : {:?}", columns);
    validate_columns(&columns)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let mut transactions: Vec<Transaction> = Vec::new();

            for result in reader.deserialize() {
                match result {
                    Ok(item) => transactions.push(to_transaction(&columns, item, account_id, fmt)?),
                    Err(e) => println!("Skipping row as unabled to process. Error: {:?}", e)
                }
            }

            if transactions_are_reversed(&transactions, account_id) {
                println!("Detected reversed transaction order, reversing...");
                transactions.reverse();
            }

            Ok(transactions)
        },
        Err(e) => {
            return Err(crate::books_error!("errors.read_csv_failure", error => e))
        }
     }
}

fn to_transaction(columns: &ColumnTypes, row: Vec<String>, account_id: Uuid, fmt: &str) -> Result<Transaction, BooksError> {
    let date = parse_date_str(get_value(&row, columns, ColumnType::Date)?, fmt)?;
    let (amount, entry_type) = determine_amount(&row, columns)?;
    let balance = get_balance(&row, columns)?;
    let entry = Entry{
        id: Uuid::new_v4(),
        transaction_id: Uuid::new_v4(),
        date: date,
        description: get_value(&row, columns, ColumnType::Description)?.to_string(),
        account_id: account_id,
        entry_type,
        amount: amount.abs(),
        balance,
        reconciled_status: None,
    };
    Ok(Transaction{ id: entry.transaction_id, entries: vec![entry], status: TransactionStatus::Recorded, source_type: None, source_id: None })
}

fn get_balance(row: &Vec<String>, columns: &ColumnTypes) -> Result<Option<Decimal>, BooksError> {
    let balance = if columns.has_column(ColumnType::Balance) {
        Some(parse_money_str(get_value(row, columns, ColumnType::Balance)?).unwrap())
    } else {
        None
    };
    Ok(balance)
}

fn determine_amount(row: &Vec<String>, columns: &ColumnTypes) -> Result<(Decimal, Side), BooksError> {

    if columns.has_column(ColumnType::Amount) {
        let amount = parse_money_str(get_value(&row, columns, ColumnType::Amount)?)?;
        Ok((amount, balance_impact(amount)))
    } else {
        let debit = parse_money_str(get_value(&row, columns, ColumnType::Debit)?)?;
        let credit = parse_money_str(get_value(&row, columns, ColumnType::Credit)?)?;
        if debit > credit {
            Ok((debit, Side::Debit))
        } else {
            Ok((credit, Side::Credit))
        }
    }

}

fn get_value(row: &Vec<String>, columns: &ColumnTypes, column: ColumnType) -> Result<String, BooksError> {
     match row.get(columns.index_of(column)) {
        Some(value) => Ok(value.to_string()),
        None => Err(crate::books_error!("errors.value_missing", field => "column"))
    }
}

pub fn balance_impact(amount: Decimal) -> Side {
    if amount.ge(&dec!(0)) {
        Side::Debit
    } else {
        Side::Credit
    }
}

pub fn read_headers<P: AsRef<Path>>(path: &P) -> Result<Vec<String> , BooksError> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let headers  = reader.headers().unwrap().iter().map(|h| h.to_string()).collect();
            Ok(headers)
        }
        Err(e) => {
            return Err(crate::books_error!("errors.read_csv_failure", error => e))
        }
    }
}

/// Read and auto detect column types from CSV file headers.
pub fn read_columns<P: AsRef<Path>>(path: &P, reverse_dr_cr: bool) -> Result<ColumnTypes, BooksError> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let headers = reader.headers().unwrap();
            detect_columns(headers, reverse_dr_cr)
        },
        Err(e) => {
            return Err(crate::books_error!("errors.read_csv_failure", error => e))
        }
    }
}

fn detect_columns(headers: &StringRecord, reverse_dr_cr: bool) -> Result<ColumnTypes, BooksError> {
    let mut columns: Vec<ColumnType> = Vec::new();
    for header in headers {
        match header.trim().to_lowercase().as_str() {
            "date" => columns.push(ColumnType::Date),
            "description" => columns.push(ColumnType::Description),
            "debit" => if reverse_dr_cr {columns.push(ColumnType::Credit)} else {columns.push(ColumnType::Debit)},
            "credit" => if reverse_dr_cr {columns.push(ColumnType::Debit)} else {columns.push(ColumnType::Credit)},
            "amount" => columns.push(ColumnType::Amount),
            "balance" => columns.push(ColumnType::Balance),
            _ => columns.push(ColumnType::Unknown),
        }
    }
    Ok(ColumnTypes { columns: columns })
}

fn validate_columns(columns: &ColumnTypes) -> Result<(), BooksError> {
    if columns.num_known_columns() == 0 {
        return Err(crate::books_error!("errors.header_missing"))
    } else if columns.num_known_columns() < 3 {
        return Err(crate::books_error!("errors.header_missing_columns"))
    }

    if columns.len() >= 3 && (columns.has_exactly_one_of(ColumnType::Amount) || (columns.has_exactly_one_of(ColumnType::Debit) && columns.has_exactly_one_of(ColumnType::Credit))) {
        return Ok(())
    }

    return Err(crate::books_error!("errors.header_missing_columns"))
}

/// Detect if transactions are in reverse chronological order (newest first)
pub fn transactions_are_reversed(transactions: &[Transaction], account_id: Uuid) -> bool {
    if transactions.len() < 2 {
        return false;
    }

    // Check if dates are generally decreasing (newest to oldest)
    let mut decreasing_count = 0;
    let mut total_comparisons = 0;
    let mut currently_decreasing = false;

    for window in transactions.windows(2) {
        if let (Some(first_entry), Some(second_entry)) = (
            window[0].find_entry_by_account(&account_id),
            window[1].find_entry_by_account(&account_id)
        ) {
            if first_entry.date > second_entry.date || (first_entry.date == second_entry.date && currently_decreasing) {
                decreasing_count += 1;
                currently_decreasing = true;
            } else {
                currently_decreasing = false;
            }
            total_comparisons += 1;
        }
    }

    // If more than 70% of comparisons show decreasing dates, consider it reversed
    println!("decreasing_count: {}, total_comparisons: {}", decreasing_count, total_comparisons);
    total_comparisons > 0 && (decreasing_count as f64 / total_comparisons as f64) > 0.7
}

fn parse_money_str(amount: String) -> Result<Decimal, BooksError> {
    let mut amount_str = amount.replace("$", "");
    amount_str = amount_str.replace(",", "");

    if amount_str.contains("(") {
        amount_str = amount_str.replace("(", "-");
        amount_str = amount_str.replace(")", "");
    }

    if amount_str.is_empty() {
        return Ok(Decimal::zero());
    }

    match Decimal::from_str(&amount_str) {
        Ok(amount) => Ok(amount),
        Err(e) => Err(crate::books_error!("errors.parse_amount_with_value", amount => amount, error => e)),
    }
}

fn parse_date_str(date_str: String, format: &str) -> Result<NaiveDate, BooksError> {
    match NaiveDate::parse_from_str(&date_str, format) {
        Ok(d) =>return Ok(d),
        Err(e) => return Err(crate::books_error!("errors.parse_date_with_format", date => date_str, format => format, error => e)),
    };
}

fn parse_money_cell<'de, D>(deserializer: D) -> Result<Decimal, BooksError>
    where D: Deserializer<'de>
{
    match String::deserialize(deserializer) {
        Ok(amount) => parse_money_str(amount),
        Err(e) => Err(crate::books_error!("errors.parse_amount", error => e)),
    }

}

fn parse_transaction_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>
{
    let date = String::deserialize(deserializer)?; // <-- this let's us skip the visitor!
    match parse_date_str(date, "%d/%m/%Y") {
        Ok(d) => return Ok(d),
        Err(e) => Err(Error::custom(e.error.as_str()))
    }
}



#[cfg(test)]
mod tests {
    use accounts::account::{Account, AccountType, Side};
    use chrono::NaiveDate;
    use csv::StringRecord;
    use rust_decimal_macros::dec;
    use crate::reader::{balance_impact, read_columns, read_transactions, ColumnType};

    use super::{check_date_format, detect_columns, parse_date_str};


    #[test]
    fn test_parse_date_str() {
        assert_eq!(NaiveDate::from_ymd_opt(2023, 10, 20).unwrap(), parse_date_str("20/10/2023".to_string(), "%d/%m/%Y").unwrap());
        assert_eq!(NaiveDate::from_ymd_opt(2023, 10, 20).unwrap(), parse_date_str("2023-10-20".to_string(), "%Y-%m-%d").unwrap());
        assert_eq!("Unable to parse date '20231020' using format '%Y-%m-%d': input contains invalid characters", parse_date_str("20231020".to_string(), "%Y-%m-%d").unwrap_err().error.as_str());
        assert_eq!("Unable to parse date '2023-13-20' using format '%Y-%m-%d': input is out of range", parse_date_str("2023-13-20".to_string(), "%Y-%m-%d").unwrap_err().error.as_str());
    }

    #[test]
    fn test_reader_file_not_found() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let columns = read_columns(&"test.csv", true).unwrap();
        let result = read_transactions(&String::from("no_such_file.csv"), account.id, "%d/%m/%Y", &columns, true);
        assert!(result.is_err());
        assert_eq!("Unable to read csv file. No such file or directory (os error 2)", result.unwrap_err().error);
    }

    #[test]
    fn test_detect_columns() {
        let headers = StringRecord::from(vec!["Date","Account","Description","Debit","Credit","Balance"]);
        let columns = detect_columns(&headers, false).unwrap();
        assert_eq!(6, columns.len());
        assert!(columns.has_column(ColumnType::Unknown));
        assert_eq!(ColumnType::Date, columns[0]);
        assert_eq!(ColumnType::Unknown, columns[1]);
        assert_eq!(ColumnType::Description, columns[2]);
        assert_eq!(ColumnType::Debit, columns[3]);
        assert_eq!(ColumnType::Credit, columns[4]);
        assert_eq!(ColumnType::Balance, columns[5]);
        println!("columns: {:?}", columns);
    }

     #[test]
    fn test_detect_columns_with_reverse_dr_cr_for_banks() {
        let headers = StringRecord::from(vec!["Date","Account","Description","Debit","Credit","Balance"]);
        let columns = detect_columns(&headers, true).unwrap();
        assert_eq!(6, columns.len());
        assert!(columns.has_column(ColumnType::Unknown));
        assert_eq!(ColumnType::Date, columns[0]);
        assert_eq!(ColumnType::Unknown, columns[1]);
        assert_eq!(ColumnType::Description, columns[2]);
        assert_eq!(ColumnType::Credit, columns[3]);
        assert_eq!(ColumnType::Debit, columns[4]);
        assert_eq!(ColumnType::Balance, columns[5]);
    }

    #[test]
    fn test_reader() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let columns = read_columns(&"test.csv", true).unwrap();
        let transactions = read_transactions(&String::from("test.csv"), account.id, "%d/%m/%Y", &columns, true).unwrap();
        assert_eq!(4, transactions.len());
        assert_eq!("Rent received", transactions[0].entries[0].description);
        assert_eq!(dec!(1200), transactions[0].entries[0].amount);
        assert_eq!(Side::Debit, transactions[0].entries[0].entry_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Credit, transactions[1].entries[0].entry_type);
    }

    #[test]
    fn test_invalid_date() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let columns = read_columns(&"test.csv", true).unwrap();
        let result = read_transactions(&String::from("test.csv"), account.id, "%Y-%M-%D", &columns, true);
        assert!(result.is_err());
        assert_eq!("Unable to parse date '31/05/2022' using format '%Y-%M-%D': input contains invalid characters", result.unwrap_err().error);
    }

    #[test]
    fn test_reader_dr_cr() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let columns = read_columns(&"test_dr_cr.csv", true).unwrap();
        let transactions = read_transactions(&String::from("test_dr_cr.csv"), account.id, "%d/%m/%Y", &columns, true).unwrap();
        assert_eq!(4, transactions.len());
        assert_eq!("Rent received", transactions[0].entries[0].description);
        assert_eq!(dec!(1200), transactions[0].entries[0].amount);
        assert_eq!(Side::Debit, transactions[0].entries[0].entry_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Credit, transactions[1].entries[0].entry_type);
    }

     #[test]
    fn test_reader_dr_cr_no_reverse() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let columns = read_columns(&"test_dr_cr.csv", false).unwrap();
        let transactions = read_transactions(&String::from("test_dr_cr.csv"), account.id, "%d/%m/%Y", &columns, true).unwrap();
        assert_eq!(4, transactions.len());
        assert_eq!("Rent received", transactions[0].entries[0].description);
        assert_eq!(dec!(1200), transactions[0].entries[0].amount);
        assert_eq!(Side::Credit, transactions[0].entries[0].entry_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Debit, transactions[1].entries[0].entry_type);
    }


    #[test]
    fn test_read_columns() {
        let columns = read_columns(&"test.csv", false).unwrap();
        assert_eq!(4, columns.len());
        assert_eq!(ColumnType::Date, columns[0]);
        assert_eq!(ColumnType::Description, columns[1]);
        assert_eq!(ColumnType::Amount, columns[2]);
        assert_eq!(ColumnType::Balance, columns[3]);
    }

    #[test]
    fn test_balance_impact() {
        let _asset_account = Account::create_new("Savings Account 1", AccountType::Asset);
        assert_eq!(Side::Debit, balance_impact(dec!(100)));
        assert_eq!(Side::Debit, balance_impact(dec!(0)));
        assert_eq!(Side::Credit, balance_impact(dec!(-100)));
    }

    #[test]
    fn test_check_date_format_dmy_slash() {
        let rows = vec![
            vec!["Date".to_string()],
            vec!["31/05/2022".to_string()],
            vec!["1/6/2022".to_string()],
            vec!["13/6/2022".to_string()],
        ];
        assert_eq!(Some("%d/%m/%Y".to_string()), check_date_format(&rows, 0));
    }

    #[test]
    fn test_check_date_format_mdy_slash() {
        let rows = vec![
            vec!["Date".to_string()],
            vec!["05/31/2022".to_string()],
            vec!["6/1/2022".to_string()],
            vec!["6/13/2022".to_string()],
        ];
        assert_eq!(Some("%m/%d/%Y".to_string()), check_date_format(&rows, 0));
    }

    #[test]
    fn test_check_date_format_ymd_dash() {
        let rows = vec![
            vec!["Posting Date".to_string()],
            vec!["2022-05-31".to_string()],
            vec!["2022-06-01".to_string()],
        ];
        assert_eq!(Some("%Y-%m-%d".to_string()), check_date_format(&rows, 0));
    }

    #[test]
    fn test_check_date_format_ymd_with_time() {
        let rows = vec![
            vec!["Date".to_string()],
            vec!["2022-05-31 00:00:00".to_string()],
            vec!["2022-06-01T12:34:56".to_string()],
        ];
        assert_eq!(Some("%Y-%m-%d".to_string()), check_date_format(&rows, 0));
    }

    #[test]
    fn test_check_date_format_ambiguous_dmy_vs_mdy() {
        let rows = vec![
            vec!["Date".to_string()],
            vec!["01/02/2022".to_string()],
            vec!["03/04/2022".to_string()],
        ];
        assert_eq!(None, check_date_format(&rows, 0));
    }

    #[test]
    fn test_check_date_format_mixed_separators_none() {
        let rows = vec![
            vec!["Date".to_string()],
            vec!["31/05/2022".to_string()],
            vec!["2022-06-01".to_string()],
        ];
        assert_eq!(None, check_date_format(&rows, 0));
    }
}
