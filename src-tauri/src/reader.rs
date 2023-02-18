use accounts::account::{Transaction, Entry, Account, TransactionStatus, Side};
use accounts::books::{BooksError, Books};
use csv::Reader;
use rust_decimal::prelude::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;
use uuid::Uuid;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use rust_decimal_macros::dec;

use serde::Deserialize;
use serde::Deserializer;


pub fn load_transactions<P: AsRef<Path>>(path: P, account: &Account) -> Result<Vec<Transaction>, BooksError> {
    let items = read_transations(path)?;
    let transactions = items.into_iter().map(|i| i.to_transaction(account)).collect();
    Ok(transactions)
}



#[derive(Debug, Deserialize)]
struct Item  {
    #[serde(deserialize_with = "parse_transaction_date")]
    date: NaiveDate,
    description: String,
    #[serde(deserialize_with = "parse_money_cell")]
    amount: Decimal,
    #[serde(deserialize_with = "parse_money_cell")]
    balance: Decimal
}

impl Item {
    fn to_transaction(&self, account: &Account) -> Transaction {
        let entry = Entry{
            id: Uuid::new_v4(),
            transaction_id: Uuid::new_v4(),
            date: self.date,
            description: self.description.clone(),
            account_id: account.id,
            transaction_type: self.balance_impact(account),
            amount: self.amount.abs(),
            balance: None
        };
        Transaction{ id: entry.transaction_id, entries: vec![entry], status: TransactionStatus::Recorded, schedule_id: None }
    }

    pub fn balance_impact(&self, account: &Account) -> Side {
        if self.amount.ge(&dec!(0)) {
            account.normal_balance()
        } else {
            account.normal_balance().opposite()
        }
    }
}

fn read_transations<P: AsRef<Path>>(path: P) -> Result<Vec<Item>, BooksError> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    match rdr {
        Ok(mut reader) => {
            let mut items: Vec<Item> = Vec::new();

            for result in reader.deserialize() {
                match result {
                    Ok(item) => {
                        println!("{:?}", item);
                        items.push(item);

                    },
                    Err(e) => {
                        println!("Skipping row as unabled to process. Error: {:?}", e)
                    },
                }
            }
            Ok(items)
        },
        Err(e) => {
            return Err(BooksError{ error: format!("Unable to read csv file. {}", e).to_string() })
        }
     }
}


fn parse_money_str(amount: &String) -> Decimal {
    let mut amount_str = amount.replace("$", "");
    amount_str = amount_str.replace(",", "");

    if amount_str.contains("(") {
        amount_str = amount_str.replace("(", "-");
        amount_str = amount_str.replace(")", "");
    }

     Decimal::from_str(&amount_str).unwrap()
}

fn parse_date_str(date_str: &String) -> NaiveDate {
    NaiveDate::parse_from_str(&date_str, "%d/%m/%Y").unwrap()
}

fn parse_money_cell<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where D: Deserializer<'de>
{
    let amount = String::deserialize(deserializer)?; // <-- this let's us skip the visitor!
    Ok(parse_money_str(&amount))
}

fn parse_transaction_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>
{
    let date = String::deserialize(deserializer)?; // <-- this let's us skip the visitor!
    Ok(parse_date_str(&date))
}



#[cfg(test)]

mod tests {
    use accounts::account::{Account, AccountType, Transaction, Side};
    use rust_decimal_macros::dec;
    use crate::reader::load_transactions;
    use super::read_transations;

    #[test]
    fn test_reader_ok() {
        let result = read_transations("test.csv");
        assert!(result.is_ok());
        assert_eq!(4, result.unwrap().len())
    }

    #[test]
    fn test_reader_file_not_found() {
        let result = read_transations("no_such_file.csv");
        assert!(result.is_err());
        assert_eq!("Unable to read csv file. No such file or directory (os error 2)", result.unwrap_err().error);
    }

    #[test]
    fn test_loader() {
        let account = Account::create_new("Savings Account 1", AccountType::Asset);
        let transactions = load_transactions("test.csv", &account).unwrap();
        println!("{:?}", transactions);
        assert_eq!(4, transactions.len());

        assert_eq!("Rent received", transactions[0].entries[0].description);
        assert_eq!(dec!(1200), transactions[0].entries[0].amount);
        assert_eq!(Side::Debit, transactions[0].entries[0].transaction_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Credit, transactions[1].entries[0].transaction_type);
    }


}