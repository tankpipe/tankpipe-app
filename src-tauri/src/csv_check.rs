use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::reader::{ColumnType, ColumnTypes};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CsvCheck {
    pub columns: ColumnTypes,
    pub header_row: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
    pub mapping_exists: bool,
    pub dr_cr_reversed: bool,
    pub date_format: Option<String>,
    pub sign_reversed_columns: HashSet<ColumnType>,
}

impl CsvCheck {
    pub fn create_new(
        columns: ColumnTypes,
        header_row: Vec<String>,
        sample_rows: Vec<Vec<String>>,
        reverse_cr_dr_default: bool,
        date_format: Option<String>,
        sign_reversed_columns: HashSet<ColumnType>,
    ) -> CsvCheck {
        let reversed = reverse_cr_dr_default
            && (columns.has_column(ColumnType::Debit)
                || columns.has_column(ColumnType::Credit));
        CsvCheck {
            columns: columns.clone(),
            header_row,
            sample_rows,
            mapping_exists: false,
            dr_cr_reversed: reversed,
            date_format,
            sign_reversed_columns,
        }
    }

    pub fn set_mapping_exists(self, mapping_exists: bool) -> CsvCheck {
        CsvCheck {
            columns: self.columns,
            header_row: self.header_row,
            sample_rows: self.sample_rows,
            mapping_exists: mapping_exists,
            dr_cr_reversed: self.dr_cr_reversed,
            date_format: self.date_format,
            sign_reversed_columns: self.sign_reversed_columns,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CsvMapping {
    pub columns: ColumnTypes,
    pub date_format: Option<String>,
    #[serde(default)]
    pub sign_reversed_columns: HashSet<ColumnType>,
}

impl CsvMapping {
    pub fn new(
        columns: Vec<String>,
        date_format: Option<String>,
        sign_reversed_columns: HashSet<ColumnType>,
    ) -> CsvMapping {
        CsvMapping {
            columns: ColumnTypes::from_vec(columns),
            date_format,
            sign_reversed_columns,
        }
    }
}
