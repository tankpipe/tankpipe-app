use accounts::account::{Transaction, AccountType, TransactionStatus, Account, ScheduleEnum, Schedule};
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
            balance: None,
			schedule_id: None			
		}
	}

}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewSchedule {
	pub name: String,
	pub period: ScheduleEnum,
	pub frequency: i64,
    #[serde(serialize_with = "serialize_naivedate")]
    #[serde(deserialize_with = "deserialize_naivedate")]
	pub start_date: NaiveDate,
    #[serde(serialize_with = "serialize_option_naivedate")]
    #[serde(deserialize_with = "deserialize_option_naivedate")]    
    pub end_date: Option<NaiveDate>,
	#[serde(serialize_with = "serialize_option_naivedate")]
    #[serde(deserialize_with = "deserialize_option_naivedate")]    
    pub last_date: Option<NaiveDate>,
	pub amount: Decimal,
	pub description: String,
	pub dr_account_id: Option<Uuid>,
	pub cr_account_id: Option<Uuid>}

impl NewSchedule {
	pub fn to_schedule(self) -> Schedule {
		Schedule {
			id: Uuid::new_v4(), 
			name: self.name,
			period: self.period,
			frequency: self.frequency,
            start_date: self.start_date, 
			end_date: self.end_date,
			last_date: None, 
			amount: self.amount,           
            description: self.description, 
			dr_account_id: self.dr_account_id, 
            cr_account_id: self.cr_account_id,   
		}
	}

}

#[derive(Clone, Serialize, Deserialize)]
pub struct DateParam {
    #[serde(serialize_with = "serialize_naivedate")]
    #[serde(deserialize_with = "deserialize_naivedate")]
	pub date: NaiveDate
}

