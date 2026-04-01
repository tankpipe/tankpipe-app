use std::{ffi::{OsStr, OsString}, path::PathBuf};
use chrono::NaiveDate;
use serde::{Deserializer, Serializer, Serialize, Deserialize};
use uuid::Uuid;
use accounts::serializer::{serialize_option_naivedate, deserialize_option_naivedate};


pub const DEFAULT_PROJECTION_MONTHS: u32 = 12;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub struct FileDetails {
    pub name: String,
    #[serde(serialize_with = "serialize_osstring")]
    #[serde(deserialize_with = "deserialize_osstring")]
    pub path: OsString,
}

impl FileDetails {
    pub fn empty() -> FileDetails {
        FileDetails{ name: "".to_string(), path: OsString::from("") }
    }

    pub fn from_path(name: &str, path: PathBuf) -> FileDetails {
        FileDetails{ name: name.to_string(), path: path.into_os_string() }
    }

    pub fn from_os_string(name: &str, path: OsString) -> FileDetails {
        FileDetails{ name: name.to_string(), path: path }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub enum Theme {
    Dark,
    Light,
    System,
}


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub enum DateFormat {
    Locale,
    Regular,
    US,
    ISO,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub struct Config {
    pub id: Uuid,
    pub version: String,
    #[serde(serialize_with = "serialize_osstring")]
    #[serde(deserialize_with = "deserialize_osstring")]
    pub data_dir: OsString,
    #[serde(serialize_with = "serialize_osstring")]
    #[serde(deserialize_with = "deserialize_osstring")]
    pub config_dir: OsString,
    pub current_file: Option<FileDetails>,
    pub current_books_id: Option<Uuid>,
    pub last_file: FileDetails,
    pub recent_files: Vec<FileDetails>,
    pub theme: Theme,
    pub display_date_format: DateFormat,
    pub import_date_format: String,

    #[serde(default = "default_projection_months")]
    pub projection_months: u32,

    #[serde(default)]
    #[serde(serialize_with = "serialize_option_naivedate")]
    #[serde(deserialize_with = "deserialize_option_naivedate")]
    pub projected_to: Option<NaiveDate>,
}

impl Config {
    pub fn data_dir_as_os_str(&self) -> &OsStr {
        &self.data_dir.as_os_str()
    }

    pub fn settings_path(&self) -> PathBuf {
        PathBuf::from(self.config_dir.clone()).join("settings.json")
    }

    pub fn file_path(&self, name: &str) -> PathBuf {
        PathBuf::from(self.data_dir.clone()).join(name)
    }

    pub fn set_last_from_path(&mut self, path: OsString, name: &str, books_id: Uuid) {
        let last_file: FileDetails;
        let os_string_path = OsString::from(path);
        self.last_file.path = os_string_path.clone();
        self.last_file.name = name.to_string();
        self.current_file = Some(self.last_file.clone());
        self.current_books_id = Some(books_id);
        let index = self.recent_files.iter().position(|f| *f.path == os_string_path);
        match index {
            Some(i) => {
                last_file = self.recent_files.get(i).unwrap().clone();
                self.recent_files.remove(index.unwrap());
            },
            None =>last_file = FileDetails::from_os_string(name, os_string_path)
        }
        self.recent_files.insert(0, last_file);
    }

    pub fn set_last(&mut self, last_file: FileDetails) {
        if let Some(index) = self.recent_files.iter().position(|f| *f.path == last_file.path) {
            self.recent_files.remove(index);
        }
        self.last_file = last_file.clone();
        self.recent_files.insert(0, last_file);
    }

}

fn default_projection_months() -> u32 {
    DEFAULT_PROJECTION_MONTHS
}

fn default_theme() -> Theme {
    Theme::System
}

pub fn deserialize_osstring<'de, D>(deserializer: D) -> Result<OsString, D::Error>
    where D: Deserializer<'de>
{
    let path = String::deserialize(deserializer)?;
    Ok(OsString::from(path))
}

pub fn serialize_osstring<S>(path: &OsString, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
{
    serializer.serialize_some(&PathBuf::from(path.as_os_str()))
}