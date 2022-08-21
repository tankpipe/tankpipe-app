use std::{ffi::{OsString, OsStr}, path::PathBuf};
use serde::{Deserializer, Serializer, Serialize, Deserialize};

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
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq  )]
pub struct Config {
    #[serde(serialize_with = "serialize_osstring")]
    #[serde(deserialize_with = "deserialize_osstring")]
    pub data_dir: OsString,
    #[serde(serialize_with = "serialize_osstring")]
    #[serde(deserialize_with = "deserialize_osstring")]
    pub config_dir: OsString,
    pub last_file: FileDetails,
    pub recent_files: Vec<FileDetails>,
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
}

pub fn deserialize_osstring<'de, D>(deserializer: D) -> Result<OsString, D::Error>
    where D: Deserializer<'de>
{
    let path = String::deserialize(deserializer)?;
    use serde::de::Error;
    Ok(OsString::from(path))
}

pub fn serialize_osstring<S>(path: &OsString, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
{
    serializer.serialize_some(&PathBuf::from(path.as_os_str()))
}