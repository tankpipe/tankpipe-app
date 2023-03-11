use accounts::account::{AccountType, Account};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use accounts::serializer::*;
use rust_decimal_macros::dec;

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

