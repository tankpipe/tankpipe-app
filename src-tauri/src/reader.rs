use accounts::account::{Transaction, Entry, Account, TransactionStatus, Side};
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

pub fn check_csv_format<P: AsRef<Path>>(path: &P) -> Result<ColumnTypes, BooksError> {
    let columns = read_columns(path)?;
    //validate_columns(&columns)?;
    Ok(columns)
}

pub fn read_rows<P: AsRef<Path>>(path: &P) -> Result<Vec<Vec<String>>, BooksError> {
    let columns = read_columns(path)?;
    validate_columns(&columns)?;
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
            return Err(BooksError{ error: format!("Unable to read csv file. {}", e).to_string() })
        }
     }
}

pub fn read_transations_using_header <P: AsRef<Path>>(path: &P, account: &Account, fmt: &str, columns: &ColumnTypes) -> Result<Vec<Transaction>, BooksError> {
    println!("read_transations_using_header : {:?}", columns);
    validate_columns(&columns)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let mut transactions: Vec<Transaction> = Vec::new();

            for result in reader.deserialize() {
                match result {
                    Ok(item) => {
                        transactions.push(to_transaction(&columns, item, account, fmt)?);

                    },
                    Err(e) => {
                        println!("Skipping row as unabled to process. Error: {:?}", e)
                    },
                }
            }
            Ok(transactions)
        },
        Err(e) => {
            return Err(BooksError{ error: format!("Unable to read csv file. {}", e).to_string() })
        }
     }
}

pub fn read_transations<P: AsRef<Path>>(path: &P, account: &Account, fmt: &str) -> Result<Vec<Transaction>, BooksError> {
    let columns = read_columns(path)?;
    validate_columns(&columns)?;
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let mut transactions: Vec<Transaction> = Vec::new();

            for result in reader.deserialize() {
                match result {
                    Ok(item) => {
                        transactions.push(to_transaction(&columns, item, account, fmt)?);

                    },
                    Err(e) => {
                        println!("Skipping row as unabled to process. Error: {:?}", e)
                    },
                }
            }
            Ok(transactions)
        },
        Err(e) => {
            return Err(BooksError{ error: format!("Unable to read csv file. {}", e).to_string() })
        }
     }
}

fn to_transaction(columns: &ColumnTypes, row: Vec<String>, account: &Account, fmt: &str) -> Result<Transaction, BooksError> {
    let date = parse_date_str(get_value(&row, columns, ColumnType::Date)?, fmt)?;
    let (amount, entry_type) = determine_amount(&row, columns, account)?;
    let entry = Entry{
        id: Uuid::new_v4(),
        transaction_id: Uuid::new_v4(),
        date: date,
        description: get_value(&row, columns, ColumnType::Description)?.to_string(),
        account_id: account.id,
        entry_type,
        amount: amount.abs(),
        balance: None
    };
    Ok(Transaction{ id: entry.transaction_id, entries: vec![entry], status: TransactionStatus::Recorded, schedule_id: None })
}

fn determine_amount(row: &Vec<String>, columns: &ColumnTypes, account: &Account) -> Result<(Decimal, Side), BooksError> {

    if columns.has_column(ColumnType::Amount) {
        let amount = parse_money_str(get_value(&row, columns, ColumnType::Amount)?)?;
        Ok((amount, balance_impact(amount, account)))
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
        None => Err(BooksError{ error: format!("Unable to find value for {}.", "column").to_string() })
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
            return Err(BooksError{ error: format!("Unable to read csv file. {}", e).to_string() })
        }
    }
}




pub fn read_columns<P: AsRef<Path>>(path: &P) -> Result<ColumnTypes, BooksError> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let headers = reader.headers().unwrap();
            detect_columns(headers, true)
        },
        Err(e) => {
            return Err(BooksError{ error: format!("Unable to read csv file. {}", e).to_string() })
        }
    }
}

fn detect_columns(headers: &StringRecord, reverse_dr_cr: bool) -> Result<ColumnTypes, BooksError> {
    let mut columns: Vec<ColumnType> = Vec::new();
    for header in headers {
        match header.to_lowercase().as_str() {
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
        return Err(BooksError{ error: "Header row not detected. First row should include headings like Date, Description, [Debit, Credit | Amount].".to_string()})
    } else if columns.num_known_columns() < 3 {
        return Err(BooksError{ error: "Header row should include one each of Date, Description, Amount[Debit, Credit | Amount] .".to_string()})
    }

    if columns.len() >= 3 && (columns.has_exactly_one_of(ColumnType::Amount) || (columns.has_exactly_one_of(ColumnType::Debit) && columns.has_exactly_one_of(ColumnType::Credit))) {
        return Ok(())
    }

    return Err(BooksError{ error: "Header row should include one each of Date, Description, [Debit, Credit | Amount].".to_string()})
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
        Err(e) => Err(BooksError{ error: format!("Unable to parse amount '{}': {}", amount, e).to_string() }),
    }
}

fn parse_date_str(date_str: String, format: &str) -> Result<NaiveDate, BooksError> {
    match NaiveDate::parse_from_str(&date_str, format) {
        Ok(d) =>return Ok(d),
        Err(e) => return Err(BooksError{error: format!("Unable to parse date '{}' using format '{}': {}", date_str, format, e).to_string()}),
    };
}

fn parse_money_cell<'de, D>(deserializer: D) -> Result<Decimal, BooksError>
    where D: Deserializer<'de>
{
    match String::deserialize(deserializer) {
        Ok(amount) => parse_money_str(amount),
        Err(e) => Err(BooksError{ error: format!("Unable to parse amount. {}", e).to_string() }),
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
    use crate::reader::{balance_impact, read_columns, read_transations_using_header, ColumnType};

    use super::{parse_date_str, read_transations, detect_columns};


    #[test]
    fn test_parse_date_str() {
        assert_eq!(NaiveDate::from_ymd(2023, 10, 20), parse_date_str("20/10/2023".to_string(), "%d/%m/%Y").unwrap());
        assert_eq!(NaiveDate::from_ymd(2023, 10, 20), parse_date_str("2023-10-20".to_string(), "%Y-%m-%d").unwrap());
        assert_eq!("Unable to parse date '20231020' using format '%Y-%m-%d': input contains invalid characters", parse_date_str("20231020".to_string(), "%Y-%m-%d").unwrap_err().error.as_str());
        assert_eq!("Unable to parse date '2023-13-20' using format '%Y-%m-%d': input is out of range", parse_date_str("2023-13-20".to_string(), "%Y-%m-%d").unwrap_err().error.as_str());
    }

    #[test]
    fn test_reader_file_not_found() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let result = read_transations(&String::from("no_such_file.csv"), &account, "%d/%m/%Y");
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
        let transactions = read_transations(&String::from("test.csv"), &account, "%d/%m/%Y").unwrap();
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
        let result = read_transations(&String::from("test.csv"), &account, "%Y-%M-%D");
        assert!(result.is_err());
        assert_eq!("Unable to parse date '31/05/2022' using format '%Y-%M-%D': input contains invalid characters", result.unwrap_err().error);

    }

    #[test]
    fn test_reader_dr_cr() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let transactions = read_transations(&String::from("test_dr_cr.csv"), &account, "%d/%m/%Y").unwrap();
        assert_eq!(4, transactions.len());
        assert_eq!("Rent received", transactions[0].entries[0].description);
        assert_eq!(dec!(1200), transactions[0].entries[0].amount);
        assert_eq!(Side::Debit, transactions[0].entries[0].entry_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Credit, transactions[1].entries[0].entry_type);
    }

    #[test]
    fn test_read_columns() {
        let columns = read_columns(&"test.csv").unwrap();
        assert_eq!(4, columns.len());
        assert_eq!(ColumnType::Date, columns[0]);
        assert_eq!(ColumnType::Description, columns[1]);
        assert_eq!(ColumnType::Amount, columns[2]);
        assert_eq!(ColumnType::Balance, columns[3]);
    }

    #[test]
    fn test_read_transactions_with_provided_columns() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let columns = read_columns(&"test.csv").unwrap();
        let transactions = read_transations_using_header(&String::from("test.csv"), &account, "%d/%m/%Y", &columns).unwrap();
        assert_eq!(4, transactions.len());
        assert_eq!("Rent received", transactions[0].entries[0].description);
        assert_eq!(dec!(1200), transactions[0].entries[0].amount);
        assert_eq!(Side::Debit, transactions[0].entries[0].entry_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Credit, transactions[1].entries[0].entry_type);
    }

    #[test]
    fn test_balance_impact() {
        let asset_account = Account::create_new("Savings Account 1", AccountType::Asset);
        assert_eq!(Side::Debit, balance_impact(dec!(100), &asset_account));
        assert_eq!(Side::Debit, balance_impact(dec!(0), &asset_account));
        assert_eq!(Side::Credit, balance_impact(dec!(-100), &asset_account));

        let credit_account = Account::create_new("Credit Card", AccountType::Liability);
        assert_eq!(Side::Debit, balance_impact(dec!(100), &credit_account));
        assert_eq!(Side::Debit, balance_impact(dec!(0), &credit_account));
        assert_eq!(Side::Credit, balance_impact(dec!(-100), &credit_account));
    }
}