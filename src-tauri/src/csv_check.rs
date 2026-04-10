use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::reader::{ColumnType, ColumnTypes};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CsvCheck {
    pub column_types: ColumnTypes,
    pub header_row: Vec<String>,
    pub sample_rows: Vec<Vec<String>>,
    pub mapping_exists: bool,
    pub dr_cr_reversed: bool,
    pub date_format: Option<String>,
    pub sign_reversed: HashSet<ColumnType>,
}

impl CsvCheck {
    pub fn create_new(
        column_types: ColumnTypes,
        header_row: Vec<String>,
        sample_rows: Vec<Vec<String>>,
        reverse_cr_dr_default: bool,
        date_format: Option<String>,
        sign_reversed: HashSet<ColumnType>,
    ) -> CsvCheck {
        let reversed = reverse_cr_dr_default
            && (column_types.has_column(ColumnType::Debit)
                || column_types.has_column(ColumnType::Credit));
        CsvCheck {
            column_types: column_types.clone(),
            header_row,
            sample_rows,
            mapping_exists: false,
            dr_cr_reversed: reversed,
            date_format,
            sign_reversed,
        }
    }

    pub fn set_mapping_exists(self, mapping_exists: bool) -> CsvCheck {
        CsvCheck {
            column_types: self.column_types,
            header_row: self.header_row,
            sample_rows: self.sample_rows,
            mapping_exists: mapping_exists,
            dr_cr_reversed: self.dr_cr_reversed,
            date_format: self.date_format,
            sign_reversed: self.sign_reversed,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CsvMapping {
    pub column_types: ColumnTypes,
    pub date_format: Option<String>,
    #[serde(default)]
    pub sign_reversed: HashSet<ColumnType>,
}

impl CsvMapping {
    pub fn new(
        columns: Vec<String>,
        date_format: Option<String>,
        sign_reversed: HashSet<ColumnType>,
    ) -> CsvMapping {
        CsvMapping {
            column_types: ColumnTypes::from_vec(columns),
            date_format,
            sign_reversed,
        }
    }
}
