#![allow(dead_code)]
use std::error::Error;
use std::ffi::{OsString, OsStr};
use std::path::PathBuf;
use std::{path::Path, fs::File, io::Read};
use std::{io, fs};
use accounts::books::{Books, BooksError};
use accounts::book_repo::{load_books, save_books};
use directories::ProjectDirs;
use tauri::api::path::home_dir;

use crate::config::{Config, FileDetails};

/// Manage storage

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppDirectories {
    data_dir: OsString,
    config_dir: OsString
}

impl AppDirectories {
    pub fn settings_path(&self) -> PathBuf {
        PathBuf::from(self.config_dir.clone()).join("settings.json")
    }

    pub fn from_project_dirs(d: &ProjectDirs) -> AppDirectories {
        AppDirectories{
            data_dir: d.data_dir().as_os_str().to_os_string(),
            config_dir: d.config_dir().as_os_str().to_os_string()
        }
    }

    pub fn to_config(&self) -> Config {
        Config{
            data_dir: self.data_dir.clone(),
            config_dir: self.config_dir.clone(),
            last_file: FileDetails::empty(),
            recent_files: Vec::new()
        }
    }
}


pub struct Repo {
    pub config: Config,
    pub books: Books,
}

impl Repo {

    pub fn load_startup() -> Result<Repo, BooksError> {
        let files = setup_app_directories()?;
        let mut config_result = load_config(files.settings_path());

        if config_result.is_err() {
            println!("Config not found so performing initial setup...");
            config_result = initial_setup();
        }
        println!("config: {:?}", config_result);
        match config_result {
            Ok(config) => {
                let result = load_books(config.last_file.path.clone());
                match result {
                    Ok(b) => {
                        Ok(Repo{ config: config, books: b })
                    },
                    Err(e) => Err(BooksError{ error: e.to_string() })
                }

            },
            Err(e) => Err(e)
        }
    }

    pub fn save(&self) -> Result<(), BooksError> {
        let _ = save_books(self.config.last_file.path.clone(), &self.books);
        Ok(())
    }
}


fn setup_app_directories() -> Result<AppDirectories, BooksError> {
    let dir = ProjectDirs::from("com", "memoneymo", "money");
    match dir {
        Some(d) => {
            let mut directories = AppDirectories::from_project_dirs(&d);
            let data_dir = fs::create_dir_all(d.data_local_dir());
            if data_dir.is_err() {
                directories.data_dir = build_home_dir_path()?;
            }
            let config_dir = fs::create_dir_all(d.config_dir());
            if config_dir.is_err() {
                directories.config_dir = OsString::from(directories.data_dir.clone())
            }
            Ok(directories)
        },
        None => {
            println!("Unable to determine directories for storing data");
            Err(BooksError::from_str("Unable to determine directories for storing data"))
        }
    }

}

fn initialise_settings(files: AppDirectories) -> Result<Config, BooksError> {
    let config = files.to_config();
    match save_config(files.settings_path(), &config) {
        Ok(_) => Ok(config),
        Err(e) => return Err(BooksError{ error: format!("Error while trying to write config file: {:?}", e) })
    }
}

fn build_home_dir_path() -> Result<OsString, BooksError> {
    let h = home_dir();
    if h.is_none() {
        return Err(BooksError{ error: "Could not determine home directory".to_string() })
    }
    Ok(h.unwrap().join("memoneymo").as_os_str().to_os_string())
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, BooksError> {
    println!("load config from: {:?}", path.as_ref());
    match File::open(path) {
        Err(why) => println!("Open settings file failed : {:?}", why.kind()),
        Ok(mut file) => {
            let mut content: String = String::new();
            if file.read_to_string(&mut content).is_ok() {
                match serde_json::from_str::<Config>(&mut content) {
                    Err(why) => println!("Parsing settings file json failed : {:?}", why),
                    Ok(config) => {
                        return Ok(config)
                    }
                }
            }
        }
    }

    Err(BooksError::from_str("Unable to load settings"))
}

fn save_config<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<()> {
    ::serde_json::to_writer(&File::create(path)?, &config)?;
    Ok(())
}

pub fn initial_setup() -> Result<Config, BooksError> {
    let files = setup_app_directories()?;
    let mut config = initialise_settings(files)?;
    let books = Books::build_empty("My Books");
    config.last_file = FileDetails::from_path("My Books", config.file_path("books1.json"));
    let _ = save_books(config.last_file.path.clone(), &books);
    match save_config(config.settings_path(), &config) {
        Ok(_) => Ok(config),
        Err(e) => return Err(BooksError{ error: format!("Error while saving config file: {:?}", e) }),
    }
}


#[cfg(test)]

mod tests {
    use serial_test::serial;
    use std::fs::File;
    use std::io::prelude::*;
    use uuid::Uuid;
    use chrono::{NaiveDate};
    use rust_decimal_macros::dec;
    use accounts::account::{Account, Transaction, AccountType, TransactionStatus, Schedule, ScheduleEnum};
    use crate::money_repo::{load_config, initial_setup, Repo};
    use super::{Books, setup_app_directories};

    #[test]
    #[serial]
    fn test_initial_setup() {
        let config = initial_setup().unwrap();
        assert!(config.data_dir.len() > 1);
        assert!(config.config_dir.len() > 1);
    }

    #[test]
    #[serial]
    fn test_load() {
        let repo = Repo::load_startup().unwrap();
        assert_eq!("My Books", repo.books.name);
    }
}