#![allow(dead_code)]
use std::collections::HashMap;
use std::ffi::{OsString};
use std::path::PathBuf;
use std::{path::Path, fs::File, io::Read};
use std::{io, fs};
use chrono::{DateTime, Datelike, Duration, Utc};
use accounts::books::{Books, BooksError};
use accounts::books_repo::{load_books, save_books, save_new_books};
use directories::ProjectDirs;
use regex::Regex;
use dirs::home_dir;
use uuid::Uuid;

use crate::config::{Config, DEFAULT_PROJECTION_MONTHS, DateFormat, FileDetails, Theme};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(test))]    
const FALLBACK_PATH: &'static str = "com.tankpipe.money";
#[cfg(test)]
const FALLBACK_PATH: &'static str = "com.tankpipe.money_test";

#[cfg(not(test))]    
const APP_NAME: &'static str = "money";
#[cfg(test)]
const APP_NAME: &'static str = "money_test";

const BACKUP_KEEP_LAST: usize = 20;
const BACKUP_KEEP_DAILY_DAYS: i64 = 31;
const BACKUP_KEEP_MONTHS: u32 = 13;


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
                            rust_i18n::t!("errors.load_books").to_string(),
                            rust_i18n::t!("errors.load_books_file", path => format!("{}", path.display())).to_string(),
                            rust_i18n::t!("errors.load_books_error", error => e).to_string()
                        ];
                        println!("{:?}", error_msgs);         

                        Err(BooksError{ error: error_msgs.join("\n") })
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn load_startup() -> Result<Repo, BooksError> {
        let mut repo = Self::load_books_with_config(|config| {
            let path = config.last_file.path.clone();                
            if path.is_empty() {
                Err(crate::books_error!("errors.no_last_file_path"))
            } else {
                Ok(path)
            }
        });

        if let Ok(ref mut repo) = repo {
            repo.run_checks();
        }

        Ok(repo?)
    }

    pub fn load_file_and_config(path: &OsString) -> Result<Repo, BooksError> {
        let mut repo = Self::load_books_with_config(|_config| Ok(path.clone()));

        if let Ok(ref mut repo) = repo {
            repo.run_checks();
        }

        Ok(repo?)
    }

    fn run_checks(&mut self) {
        let today = chrono::Utc::now().date_naive();
        let new_projection_date = today.checked_add_months(chrono::Months::new(self.config.projection_months)).unwrap();
        let _ = self.books.run_checks_and_update(new_projection_date);
        self.config.projected_to = Some(new_projection_date);
        if self.config.current_file.is_some() {
            let _ = save_books_with_backups(self.config.current_file.clone().unwrap().path.clone(), &self.books);
        }
        let _ = self.save_config();
    }

    pub fn check_interest(&mut self) {
        if self.books.interest_outdated() {
            let today = chrono::Utc::now().date_naive();
            let new_projection_date = today.checked_add_months(chrono::Months::new(self.config.projection_months)).unwrap();
            let _ = self.books.recalculate_interest(new_projection_date);
        }
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
                    let result = save_books_with_backups(self.config.current_file.clone().unwrap().path.clone(), &self.books);
                    
                    match result {
                        Ok(()) => {
                            println!("Saved books to {:?}", self.config.current_file.clone().unwrap().path.clone());
                            Ok(())
                        },
                        Err(e) => {
                            println!("Failed to save books: {}", e);
                            Err(BooksError{ error: e.to_string() })
                        }
                    }
                } else {
                    Err(crate::books_error!("errors.current_books_id_mismatch"))
                }
            },
            None => Err(crate::books_error!("errors.no_file_path_for_current_books"))
        }
    }

    pub fn list_backups(&self) -> Result<Vec<OsString>, BooksError> {
        match &self.config.current_file {
            Some(file_details) => {
                let current_path = PathBuf::from(file_details.path.clone());
                let backup_dir = backup_dir_for_file(&current_path).map_err(|e| BooksError{ error: e.to_string() })?;
                if !backup_dir.exists() {
                    return Ok(Vec::new());
                }

                let mut entries = read_backup_entries(&backup_dir).map_err(|e| BooksError{ error: e.to_string() })?;
                entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                Ok(entries.into_iter().map(|entry| entry.path.into_os_string()).collect())
            },
            None => Err(crate::books_error!("errors.no_file_path_for_current_books"))
        }
    }

    pub fn restore_backup(&mut self, backup_path: &OsString) -> Result<(), BooksError> {
        let current_file = match &self.config.current_file {
            Some(file) => file.clone(),
            None => return Err(crate::books_error!("errors.no_file_path_for_current_books"))
        };

        let current_path = PathBuf::from(current_file.path.clone());
        let backup_path_buf = PathBuf::from(backup_path.clone());
        let backup_dir = backup_dir_for_file(&current_path).map_err(|e| BooksError{ error: e.to_string() })?;

        let backup_dir_canonical = backup_dir.canonicalize().map_err(|e| BooksError{ error: e.to_string() })?;
        let backup_path_canonical = backup_path_buf.canonicalize().map_err(|e| BooksError{ error: e.to_string() })?;
        if !backup_path_canonical.starts_with(&backup_dir_canonical) {
            return Err(BooksError{ error: "Backup file is not in this repo's backup directory.".to_string() })
        }

        let restored_books = load_books(backup_path.clone()).map_err(|e| BooksError{ error: e.to_string() })?;
        save_books_with_backups(current_file.path.clone(), &restored_books).map_err(|e| BooksError{ error: e.to_string() })?;

        self.books = restored_books;
        self.config.current_books_id = Some(self.books.id);
        self.save_config()?;
        Ok(())
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
            Err(e) => return Err(crate::books_error!("errors.save_config_error", error => format!("{:?}", e))),
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
            Err(e) => return Err(crate::books_error!("errors.save_config_error", error => format!("{:?}", e))),
        }
    }
     
    pub fn save_config(&self) -> Result<(), BooksError> {
        match write_config(self.config.settings_path(), &self.config) {
            Ok(_) => Ok(()),
            Err(e) => return Err(crate::books_error!("errors.save_config_error", error => format!("{:?}", e))),
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
            theme: Theme::System,
            display_date_format: DateFormat::Locale,
            import_date_format: "%d/%m/%Y".to_string(),
            csv_mappings: HashMap::new(),
            projection_months: DEFAULT_PROJECTION_MONTHS,
            projected_to: None,
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
    let dir = ProjectDirs::from("com", "tankpipe", APP_NAME);

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
            println!("Unable to determine directories for storing testdata");
            Err(crate::books_error!("errors.determine_directories_failure"))
        }
    }

}

fn initialise_settings(files: AppDirectories) -> Result<Config, BooksError> {
    let config = files.to_config();
    match write_config(files.settings_path(), &config) {
        Ok(_) => Ok(config),
        Err(e) => return Err(crate::books_error!("errors.write_config_error", error => format!("{:?}", e)))
    }
}   

fn build_home_dir_path() -> Result<OsString, BooksError> {
    let h = home_dir();
    if h.is_none() {
        return Err(crate::books_error!("errors.determine_home_directory_failure"))
    }       
    Ok(h.unwrap().join(FALLBACK_PATH).as_os_str().to_os_string())
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

    Err(crate::books_error!("errors.load_settings_failure"))
}

fn write_config<P: AsRef<Path>>(path: P, config: &Config) -> io::Result<()> {
    ::serde_json::to_writer(&File::create(path)?, &config)?;
    Ok(())
}

#[derive(Clone, Debug)]
struct BackupEntry {
    path: PathBuf,
    timestamp: DateTime<Utc>
}

fn save_books_with_backups(path: OsString, books: &Books) -> io::Result<()> {
    create_backup_snapshot(path.as_os_str())?;
    save_books(path, books)
}

fn create_backup_snapshot(path: &std::ffi::OsStr) -> io::Result<()> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Ok(())
    }

    let backup_dir = backup_dir_for_file(&path)?;
    fs::create_dir_all(&backup_dir)?;

    let now = Utc::now();
    let mut backup_path = backup_dir.join(format!("backup-{}.json", now.timestamp_millis()));
    let mut collision = 1u32;
    while backup_path.exists() {
        backup_path = backup_dir.join(format!("backup-{}-{}.json", now.timestamp_millis(), collision));
        collision += 1;
    }

    fs::copy(&path, &backup_path)?;
    prune_backups(&backup_dir, now)
}

fn backup_dir_for_file(path: &Path) -> io::Result<PathBuf> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path must point to a file"))?;
    Ok(parent.join(".backups").join(file_name))
}

fn prune_backups(backup_dir: &Path, now: DateTime<Utc>) -> io::Result<()> {
    let mut entries = read_backup_entries(backup_dir)?;
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if entries.len() <= BACKUP_KEEP_LAST {
        return Ok(())
    }

    let daily_cutoff = now - Duration::days(BACKUP_KEEP_DAILY_DAYS);
    let recent_months = recent_month_keys(now, BACKUP_KEEP_MONTHS);
    let mut keep_paths: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
    let mut kept_days: std::collections::HashSet<(i32, u32, u32)> = std::collections::HashSet::new();
    let mut kept_months: std::collections::HashSet<(i32, u32)> = std::collections::HashSet::new();

    for backup in entries.iter().take(BACKUP_KEEP_LAST) {
        keep_paths.insert(backup.path.clone());
    }

    for backup in &entries {
        if backup.timestamp >= daily_cutoff {
            let day_key = (backup.timestamp.year(), backup.timestamp.month(), backup.timestamp.day());
            if kept_days.insert(day_key) {
                keep_paths.insert(backup.path.clone());
            }
        }
    }

    for backup in &entries {
        let month_key = (backup.timestamp.year(), backup.timestamp.month());
        if recent_months.contains(&month_key) && kept_months.insert(month_key) {
            keep_paths.insert(backup.path.clone());
        }
    }

    for backup in entries {
        if !keep_paths.contains(&backup.path) {
            fs::remove_file(&backup.path)?;
        }
    }

    Ok(())
}

fn read_backup_entries(backup_dir: &Path) -> io::Result<Vec<BackupEntry>> {
    let mut backups = Vec::new();
    for dir_entry in fs::read_dir(backup_dir)? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };
        let timestamp_ms = match parse_backup_timestamp_ms(file_name) {
            Some(ms) => ms,
            None => continue,
        };
        let timestamp = match DateTime::<Utc>::from_timestamp_millis(timestamp_ms) {
            Some(ts) => ts,
            None => continue,
        };

        backups.push(BackupEntry { path, timestamp });
    }
    Ok(backups)
}

fn parse_backup_timestamp_ms(file_name: &str) -> Option<i64> {
    if !file_name.starts_with("backup-") || !file_name.ends_with(".json") {
        return None;
    }
    let without_prefix = &file_name["backup-".len()..file_name.len() - ".json".len()];
    let millis_str = without_prefix.split('-').next()?;
    millis_str.parse::<i64>().ok()
}

fn recent_month_keys(now: DateTime<Utc>, months: u32) -> std::collections::HashSet<(i32, u32)> {
    let mut month_keys = std::collections::HashSet::new();
    let mut year = now.year();
    let mut month = now.month();

    for _ in 0..months {
        month_keys.insert((year, month));
        if month == 1 {
            month = 12;
            year -= 1;
        } else {
            month -= 1;
        }
    }

    month_keys
}

pub fn initial_setup() -> Result<Config, BooksError> {
    let files = setup_app_directories()?;
    let config = initialise_settings(files)?;
    match write_config(config.settings_path(), &config) {
        Ok(_) => Ok(config),
        Err(e) => return Err(crate::books_error!("errors.save_config_error", error => format!("{:?}", e))),
    }
}


#[cfg(test)]
mod tests {
    use std::{ffi::OsString, path::PathBuf};

    use accounts::books_repo::file_exists;
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
