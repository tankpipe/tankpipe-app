use serde::{Serialize, Deserialize};


use crate::reader::ColumnTypes;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CsvCheck {
    pub column_types: ColumnTypes,
    pub header_row: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
}

impl CsvCheck {
    pub fn create_new(column_types: ColumnTypes, header_row: Vec<String>, sample_rows: Vec<Vec<String>>) -> CsvCheck {
        CsvCheck {
            column_types: column_types,
            header_row: header_row,
            sample_rows: sample_rows,
        }
    }

}


