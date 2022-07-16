use accounts::account::{Transaction, AccountType, TransactionStatus};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use accounts::serializer::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
	pub name: String,
	pub account_type: AccountType,
	pub balance: Decimal,
	pub starting_balance: Decimal
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewTransaction {
    #[serde(serialize_with = "serialize_naivedate")]
    #[serde(deserialize_with = "deserialize_naivedate")]
	pub date: NaiveDate,
	pub description: String,
	pub dr_account_id:   Option<Uuid>,
    pub cr_account_id:   Option<Uuid>,
	pub amount:      Decimal,
	pub status:      TransactionStatus,
}

impl NewTransaction {
	pub fn to_transaction(self) -> Transaction {
		Transaction{
			id: Uuid::new_v4(), 
            date: self.date, 
            description: self.description, 
            dr_account_id: self.dr_account_id, 
            cr_account_id: self.cr_account_id, 
            amount: self.amount, 
            status: self.status,
            balance: None			
		}
	}

}



