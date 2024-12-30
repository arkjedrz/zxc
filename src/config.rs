use std::fs::create_dir;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// `zxc` app configuration.
pub struct Config {
    /// Current working directory.
    pub cwd: PathBuf,
    /// `zxc` app home directory.
    pub app_home: PathBuf,
}

impl Config {
    /// Create configuration.
    /// If `$HOME/.zxc` directory doesn't exist - create such.
    pub fn new() -> Result<Self, Error> {
        // Get current working directory.
        let cwd = match std::env::current_dir() {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        // Get `$HOME`.
        let home_env = match std::env::var("HOME") {
            Ok(value) => value,
            Err(_) => return Err(Error::new(ErrorKind::NotFound, "$HOME not found")),
        };

        // Get app home - `$HOME/.zxc`.
        let app_home = PathBuf::from(home_env).join(".zxc");

        // Create app home if not found.
        if !app_home.exists() {
            match create_dir(&app_home) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(Config { cwd, app_home })
    }
}
