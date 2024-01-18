use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use lazy_static::lazy_static;
use log::error;
use serde::{Deserialize, Serialize};
use crate::db::DBEngine;

lazy_static! {
    pub static ref CONFIG: Config = {
        let dir = env::current_dir().expect("");
        let config_path: PathBuf = dir.join("config.yaml");

        if let Err(_) = std::fs::metadata(&config_path) {
            File::create(&config_path).expect("Failed to create config.");
        }

        let mut file = File::open(&config_path).expect("Failed to open file.");

        let mut s = String::from("");
        file.read_to_string(&mut s).expect("Failed to read config.");

        serde_yaml::from_str(s.as_str()).expect("Failed to parse config.")
    };
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub db: DBConfig,
    pub log: Option<LoggerConfig>,
    pub redis: RedisConfig,
    pub email: EmailConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DBConfig {
    pub username: String,
    pub password: String,
    pub db_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoggerConfig {
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EmailConfig {
    pub server: String,
    pub port: u16,
    pub from: String,
    pub protocol: String,
    pub password: String,
}