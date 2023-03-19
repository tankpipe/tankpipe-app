use accounts::account::{Transaction, Entry, Account, TransactionStatus, Side};
use accounts::books::{BooksError};
use rust_decimal::prelude::*;
use chrono::{NaiveDate};
use serde::de::Error;
use uuid::Uuid;
use std::path::{Path};
use rust_decimal_macros::dec;

use serde::Deserialize;
use serde::Deserializer;


pub fn load_transactions<P: AsRef<Path>>(path: P, account: &Account) -> Result<Vec<Transaction>, BooksError> {
    let mut transactions: Vec<Transaction> = Vec::new();
    for item in read_transations(path)? {
        transactions.push(item.to_transaction(account)?);
    }
    Ok(transactions)
}



#[derive(Debug, Deserialize)]
struct Item  {
    date_str: String,
    description: String,
    #[serde(deserialize_with = "parse_money_cell")]
    amount: Decimal,
    #[serde(deserialize_with = "parse_money_cell")]
    balance: Decimal
}

impl Item {
    fn to_transaction(&self, account: &Account) -> Result<Transaction, BooksError> {
        let date = parse_date_str(&self.date_str, "%d/%m/%Y")?;
        let entry = Entry{
            id: Uuid::new_v4(),
            transaction_id: Uuid::new_v4(),
            date: date,
            description: self.description.clone(),
            account_id: account.id,
            entry_type: self.balance_impact(account),
            amount: self.amount.abs(),
            balance: None
        };
        Ok(Transaction{ id: entry.transaction_id, entries: vec![entry], status: TransactionStatus::Recorded, schedule_id: None })
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

fn parse_date_str(date_str: &String, format: &str) -> Result<NaiveDate, BooksError> {
    match NaiveDate::parse_from_str(&date_str, format) {
        Ok(d) =>return Ok(d),
        Err(e) => return Err(BooksError{error: format!("Unable to parse date: {}", e).to_string()}),
    };
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
    match parse_date_str(&date, "%d/%m/%Y") {
        Ok(d) => return Ok(d),
        Err(e) => Err(Error::custom(e.error.as_str()))
    }
}



#[cfg(test)]

mod tests {
    use accounts::account::{Account, AccountType, Side};
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;
    use crate::reader::load_transactions;
    use super::{read_transations, parse_date_str};


    #[test]
    fn test_parse_date_str() {
        assert_eq!(NaiveDate::from_ymd(2023, 10, 20), parse_date_str(&"20/10/2023".to_string(), "%d/%m/%Y").unwrap());
        assert_eq!(NaiveDate::from_ymd(2023, 10, 20), parse_date_str(&"2023-10-20".to_string(), "%Y-%m-%d").unwrap());
        assert_eq!("Unable to parse date: input contains invalid characters", parse_date_str(&"20231020".to_string(), "%Y-%m-%d").unwrap_err().error.as_str());
        assert_eq!("Unable to parse date: input is out of range", parse_date_str(&"2023-13-20".to_string(), "%Y-%m-%d").unwrap_err().error.as_str());
    }

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
        assert_eq!(Side::Debit, transactions[0].entries[0].entry_type);
        assert_eq!(account.id, transactions[0].entries[0].account_id);
        assert_eq!(dec!(500), transactions[1].entries[0].amount);
        assert_eq!(Side::Credit, transactions[1].entries[0].entry_type);
    }


}