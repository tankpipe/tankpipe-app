use accounts::account::{AccountType, Account};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use accounts::serializer::*;
use rust_decimal_macros::dec;

use crate::{config::DateFormat, money_repo::Repo};

#[derive(Clone, Serialize, Deserialize)]
pub struct NewAccount {
    pub name: String,
    pub account_type: AccountType,
    pub starting_balance: Decimal
}

impl NewAccount {
    pub fn to_account(self) -> Account {
        Account {
            id: Uuid::new_v4(),
            name: self.name,
            starting_balance: self.starting_balance,
            account_type: self.account_type,
            balance: dec!(0)
        }
    }

}


#[derive(Clone, Serialize, Deserialize)]
pub struct DateParam {
    #[serde(serialize_with = "serialize_naivedate")]
    #[serde(deserialize_with = "deserialize_naivedate")]
    pub date: NaiveDate
}


#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigSettings {
    pub display_date_format: DateFormat,
    pub import_date_format: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Analytics {
    pub id: Uuid,
    pub num_books: usize,
    pub num_accounts: usize,
    pub num_schedules: usize,
    pub num_transactions: usize,
}

impl Analytics {
    pub fn from_repo(repo: &Repo) -> Analytics {
        Analytics {
            id: repo.config.id, num_books:
            repo.config.recent_files.len(),
            num_accounts: repo.books.accounts().len(),
            num_schedules: repo.books.schedules().len(),
            num_transactions: repo.books.transactions().len()
        }
    }
}