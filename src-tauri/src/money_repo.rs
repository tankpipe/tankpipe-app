#![allow(dead_code)]
use std::collections::HashMap;
use std::ffi::{OsString};
use std::path::PathBuf;
use std::{path::Path, fs::File, io::Read};
use std::{io, fs};
use accounts::books::{Books, BooksError};
use accounts::book_repo::{load_books, save_books, save_new_books};
use directories::ProjectDirs;
use regex::Regex;
use dirs::home_dir;
use uuid::Uuid;

use crate::config::{Config, FileDetails, DateFormat};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Manage storage

pub struct Repo {
    pub config: Config,
    pub books: Books
}

impl Repo {

    pub fn from_components(config: Config, books: Books) -> Repo {
        Repo{ config: config, books: books}
    }
    
    fn load_books_with_config<F>(path_provider: F) -> Result<Repo, BooksError> 
    where 
        F: FnOnce(&Config) -> Result<OsString, BooksError>
    {
        let config_result = Repo::load_config();
        println!("config: {:?}", config_result);
        match config_result {
            Ok(mut config) => {
                let path = path_provider(&config)?;
                let result = load_books(path.clone());
                match result {
                    Ok(b) => {
                        config.set_last_from_path(path.clone(), b.name.clone().as_str(), b.id);
                        let _save_result = write_config(&config.settings_path(), &config);
                        Ok(Repo::from_components(config, b))
                    },
                    Err(e) => {
                        let error_msgs = vec![
                            format!("Error while loading the books file."),
                            format!("File: {:?}", &path.display()),
                            format!("Error: {}", e )];
                        println!("{:?}", error_msgs);         

                        Err(BooksError{ error: error_msgs.join("\n") })
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn load_startup() -> Result<Repo, BooksError> {
        Self::load_books_with_config(|config| {
            let path = config.last_file.path.clone();                
            if path.is_empty() {
                Err(BooksError{ error: "No last file path.".to_string() })
            } else {
                Ok(path)
            }
        })
    }

    pub fn load_file_and_config(path: &OsString) -> Result<Repo, BooksError> {
        Self::load_books_with_config(|_config| Ok(path.clone()))
    }

    pub fn load_config() -> Result<Config, BooksError> {
        let files = setup_app_directories()?;
        let mut config_result = read_config(files.settings_path());

        if config_result.is_err() {
            println!("Config not found so performing initial setup...");
            config_result = initial_setup();
        }
        config_result
    }

    pub fn load_books(&mut self, path: &OsString) -> Result<(), BooksError> {
        let result = load_books(path);

        match result {
            Ok(b) => {
                self.books = b;
                self.config.set_last_from_path(path.clone(), self.books.name.clone().as_str(), self.books.id);
                let save_result = write_config( &self.config.settings_path(), &self.config);
                match save_result {
                    Ok(()) => Ok(()),
                    Err(e) => Err(BooksError{ error: e.to_string() })
                }
            },
            Err(e) => Err(BooksError{ error: e.to_string() })
        }

    }

    pub fn save(&self) -> Result<(), BooksError> {
        
        match self.config.current_books_id  {
            Some(id) => {                
                if id == self.books.id {
                    let _ = save_books(self.config.current_file.clone().unwrap().path.clone(), &self.books);
                    Ok(())
                } else {
                    Err(BooksError::from_str("Current books id does not match the file path books id"))
                }
            },
            None => Err(BooksError::from_str("No file path for current books"))
        }
    }

    pub fn new_books(&mut self, name: &str) -> Result<(), BooksError> {
        self.books = Books::build_empty(name);
        let re = Regex::new(r"[^a-z0-9_\-]").unwrap();
        let lower_name = name.to_ascii_lowercase();
        let file_name = format!("{}.json", re.replace_all(&lower_name, "_"));
        let last_file = FileDetails::from_path(name, self.config.file_path(&file_name));
        self.config.set_last(last_file.clone());
        self.config.current_books_id = Some(self.books.id);
        self.config.current_file = Some(last_file.clone());
        save_new_books(self.config.last_file.path.clone(), &self.books)?;
        match write_config(self.config.settings_path(), &self.config) {
            Ok(_) => Ok(()),
            Err(e) => return Err(BooksError{ error: format!("Error while saving config file: {:?}", e) }),
        }
    }

    pub fn save_new_repo(&mut self) -> Result<(), BooksError> {
        let file_name = derive_file_name(&self.books.name);
        let last_file = FileDetails::from_path(&self.books.name.clone(), self.config.file_path(&file_name));
        self.config.set_last(last_file.clone());
        self.config.current_books_id = Some(self.books.id);
        self.config.current_file = Some(last_file.clone());
        save_new_books(self.config.last_file.path.clone(), &self.books)?;
        
        match write_config(self.config.settings_path(), &self.config) {
            Ok(_) => Ok(()),
            Err(e) => return Err(BooksError{ error: format!("Error while saving config file: {:?}", e) }),
        }
    }
     
    pub fn save_config(&self) -> Result<(), BooksError> {
        match write_config(self.config.settings_path(), &self.config) {
            Ok(_) => Ok(()),
            Err(e) => return Err(BooksError{ error: format!("Error while saving config file: {:?}", e) }),
        }
    }

    pub fn first_repo(name: &str) -> Result<Repo, BooksError> {
        let mut config =  Repo::load_config()?;   
        let books = Books::build_empty(&name);
        config.current_books_id = Some(books.id);
        let mut repo = Repo::from_components(config, books);
        repo.save_new_repo()?;
        Ok(repo)
    }

}

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
            id: Uuid::new_v4(),
            version: VERSION.to_string(),
            data_dir: self.data_dir.clone(),
            config_dir: self.config_dir.clone(),
            current_books_id: None,
            current_file: None,
            last_file: FileDetails::empty(),
            recent_files: Vec::new(),
            display_date_format: DateFormat::Locale,
            import_date_format: "%d/%m/%Y".to_string(),
            csv_mappings: HashMap::new(),
        }
    }
}

pub fn derive_file_name(name: &str) -> String {
    let re = Regex::new(r"[^a-z0-9_\-]").unwrap();
    let lower_name = name.to_ascii_lowercase();
    let file_name = format!("{}.json", re.replace_all(&lower_name, "_"));
    file_name
}


fn setup_app_directories() -> Result<AppDirectories, BooksError> {
    let dir = ProjectDirs::from("com", "tankpipe", "money");
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
    match write_config(files.settings_path(), &config) {
        Ok(_) => Ok(config),
        Err(e) => return Err(BooksError{ error: format!("Error while trying to write config file: {:?}", e) })
    }
}

fn build_home_dir_path() -> Result<OsString, BooksError> {
    let h = home_dir();
    if h.is_none() {
        return Err(BooksError{ error: "Could not determine home directory".to_string() })
    }
    Ok(h.unwrap().join("com.tankpipe.money").as_os_str().to_os_string())
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, BooksError> {
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

fn write_config<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<()> {
    ::serde_json::to_writer(&File::create(path)?, &config)?;
    Ok(())
}

pub fn initial_setup() -> Result<Config, BooksError> {
    let files = setup_app_directories()?;
    let config = initialise_settings(files)?;
    match write_config(config.settings_path(), &config) {
        Ok(_) => Ok(config),
        Err(e) => return Err(BooksError{ error: format!("Error while saving config file: {:?}", e) }),
    }
}


#[cfg(test)]

mod tests {
    use std::{ffi::OsString, path::PathBuf};

    use accounts::book_repo::file_exists;
    use serial_test::serial;
    use crate::money_repo::{Repo, derive_file_name, initial_setup, setup_app_directories};

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
        let name = "Unit Test Books";
        let app_dirs = setup_app_directories().unwrap();
        let file_path = PathBuf::from(app_dirs.data_dir.clone()).join(derive_file_name(name));

        if file_exists(OsString::from(file_path.clone())) {
            std::fs::remove_file(OsString::from(file_path.clone())).unwrap();
        }

        let repo = Repo::first_repo(name).unwrap();
        assert_eq!(name, repo.books.name);
        assert_eq!(repo.books.id, repo.config.current_books_id.unwrap());
        std::fs::remove_file(OsString::from(file_path.clone())).unwrap();
    }
}