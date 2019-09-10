/*
 * config.rs
 *
 * kant-router - Wikidot-compatible router for web applications
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use log::LevelFilter;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Info;

// Structopt argument parsing

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kant-router",
    about = "Wikidot-compatible router for web applications"
)]
struct Options {
    /// Logging level to use.
    #[structopt(short, long)]
    level: Option<LevelFilter>,

    /// Configuration file.
    #[structopt(name = "CONFIG_FILE", parse(from_os_str))]
    config_file: PathBuf,
}

// Configuration objects

#[derive(Debug, Clone)]
pub struct Config {
    pub log_level: LevelFilter,
    pub http_port: u16,
    pub https_port: u16,
}

impl Config {
    #[cold]
    pub fn parse_args() -> Self {
        let opts = Options::from_args();
        let mut config: Self = ConfigFile::read(&opts.config_file).into();
        if let Some(level) = opts.level {
            config.log_level = level;
        }

        config
    }
}

#[serde(rename_all = "kebab-case")]
#[derive(Deserialize, Debug)]
struct ConfigFile {
    pub log_level: Option<String>,
    pub http_port: Option<u16>,
    pub https_port: Option<u16>,
}

impl ConfigFile {
    #[cold]
    fn read(path: &Path) -> Self {
        let mut file = File::open(path).expect("Unable to open config file");
        let mut contents = String::new();
        let _ = file
            .read_to_string(&mut contents)
            .expect("Unable to read config file");
        let obj: Self = toml::from_str(&contents).expect("Unable to parse TOML in config file");

        obj
    }

    #[cold]
    fn parse_log_level(&self) -> LevelFilter {
        const LEVELS: [(&str, LevelFilter); 9] = [
            ("", DEFAULT_LOG_LEVEL),
            ("off", LevelFilter::Off),
            ("none", LevelFilter::Off),
            ("trace", LevelFilter::Trace),
            ("debug", LevelFilter::Debug),
            ("warn", LevelFilter::Warn),
            ("warning", LevelFilter::Warn),
            ("err", LevelFilter::Error),
            ("error", LevelFilter::Error),
        ];

        let log_level = match self.log_level {
            Some(ref log_level) => log_level,
            None => return DEFAULT_LOG_LEVEL,
        };

        for (text, level) in &LEVELS {
            if log_level.eq_ignore_ascii_case(text) {
                return *level;
            }
        }

        panic!("No such log level for '{}'", log_level);
    }
}

impl Into<Config> for ConfigFile {
    #[cold]
    fn into(self) -> Config {
        let log_level = self.parse_log_level();
        let http_port = self.http_port.unwrap_or(80);
        let https_port = self.https_port.unwrap_or(443);

        Config {
            log_level,
            http_port,
            https_port,
        }
    }
}
