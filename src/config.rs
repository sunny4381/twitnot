use std::env;
use std::fs;
use std::io::Write;
use std::path;

use serde_json::{self, json};

use crate::error::Error;

#[derive(Debug)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub gmail_command: String,
    pub database_file: String,
    pub notification_from_email: String,
    pub notification_tos: Vec<String>,
}

impl Config {
    pub fn home_dir() -> Result<path::PathBuf, env::VarError> {
        match env::var("TWITNOT_HOME") {
            Ok(path) => return Ok(path::PathBuf::from(path)),
            Err(_) => (),
        };

        let home = env::var("HOME")?;
        let mut homepath = path::PathBuf::from(home);
        homepath.push(".twitnot");
        return Ok(homepath);
    }

    pub fn default_database_file() -> Result<String, Error> {
        let config_dir = Config::home_dir()?;
        Ok(String::from(config_dir.as_path().join("default.sqlite3").to_str().unwrap()))
    }

    pub fn load(profile: &str) -> Result<Config, Error> {
        let config_dir = Self::home_dir()?;
        let filepath = config_dir.as_path().join(profile);
        let file = fs::File::open(filepath)?;

        let cfg: serde_json::Value = serde_json::from_reader(file)?;
        let str_val = |key: &'static str| cfg[key].as_str().map(String::from).ok_or(Error::ConfigError(key));
        let consumer_key = str_val("consumer_key")?;
        let consumer_secret = str_val("consumer_secret")?;
        let gmail_command = str_val("gmail_command")?;
        let database_file = str_val("database_file")?;
        let notification_from_email = str_val("notification_from_email")?;
        let notification_tos: Vec<String> = if let Some(ary) = cfg["notification_tos"].as_array() {
            ary.into_iter().filter_map(|item| item.as_str().map(String::from)).collect()
        } else {
            vec![]
        };

        return Ok(Config {
            consumer_key: consumer_key,
            consumer_secret: consumer_secret,
            gmail_command: gmail_command,
            database_file: database_file,
            notification_from_email: notification_from_email,
            notification_tos: notification_tos,
        });
    }

    pub fn save(&self, profile: &str) -> Result<(), Error> {
        let cfg = json!({
            "consumer_key": self.consumer_key,
            "consumer_secret": self.consumer_secret,
            "gmail_command": self.gmail_command,
            "database_file": self.database_file,
            "notification_from_email": self.notification_from_email,
            "notification_tos": self.notification_tos,
        });

        let config_dir = Self::home_dir()?;
        fs::create_dir_all(config_dir.as_path())?;

        let filepath = config_dir.as_path().join(profile);
        let mut file = fs::File::create(filepath)?;

        file.write_all(cfg.to_string().as_bytes())?;

        return Ok(());
    }
}
