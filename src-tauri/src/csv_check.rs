use serde::{Serialize, Deserialize};


use crate::reader::ColumnTypes;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CsvCheck {
    pub column_types: ColumnTypes,
    pub header_row: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
    pub mapping_exists: bool,
}

impl CsvCheck {
    pub fn create_new(column_types: ColumnTypes, header_row: Vec<String>, sample_rows: Vec<Vec<String>>) -> CsvCheck {
        CsvCheck {
            column_types: column_types,
            header_row: header_row,
            sample_rows: sample_rows,
            mapping_exists: false,
        }
    }

    pub fn set_mapping_exists(self, mapping_exists: bool) -> CsvCheck {
        CsvCheck {
            column_types: self.column_types,
            header_row: self.header_row,
            sample_rows: self.sample_rows,
            mapping_exists: mapping_exists,
        }
    }
}


