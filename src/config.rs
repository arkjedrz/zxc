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
        let cwd = std::env::current_dir()?;

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
        } else if !app_home.is_dir() {
            return Err(Error::from(ErrorKind::NotADirectory));
        }

        Ok(Config { cwd, app_home })
    }
}

#[cfg(test)]
mod config_tests {
    // Tests must be run serially, as they interact with filesystem and env vars.
    use super::Config;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use std::io::ErrorKind;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn create_app_home() {
        let temp_home = tempdir().unwrap();
        let home_path = temp_home.path();
        env::set_var("HOME", home_path);

        let zxc_path = home_path.join(".zxc");
        assert!(!zxc_path.exists());

        let config = Config::new().unwrap();
        assert_eq!(config.cwd, env::current_dir().unwrap());
        assert_eq!(config.app_home, zxc_path);
        assert!(zxc_path.exists() && zxc_path.is_dir());
    }

    #[test]
    #[serial]
    fn create_app_home_fail() {
        let temp_home = tempdir().unwrap();
        let home_path = temp_home.path();
        env::set_var("HOME", home_path);

        let zxc_path = home_path.join(".zxc");
        fs::File::create(&zxc_path).unwrap();
        assert!(zxc_path.exists() && zxc_path.is_file());

        let result = Config::new();
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::NotADirectory));

        fs::remove_file(zxc_path).unwrap();
    }

    #[test]
    #[serial]
    fn app_home_exists() {
        let temp_home = tempdir().unwrap();
        let home_path = temp_home.path();
        env::set_var("HOME", home_path);

        let zxc_path = home_path.join(".zxc");
        fs::create_dir(&zxc_path).unwrap();
        assert!(zxc_path.exists() && zxc_path.is_dir());

        let config = Config::new().unwrap();
        assert_eq!(config.cwd, env::current_dir().unwrap());
        assert_eq!(config.app_home, zxc_path);
    }

    #[test]
    #[serial]
    fn missing_home_env() {
        let original_home = env::var("HOME").unwrap();
        env::remove_var("HOME");

        let result = Config::new();
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::NotFound));

        env::set_var("HOME", original_home);
    }

    #[test]
    #[serial]
    fn invalid_current_dir() {
        let original_cwd = env::current_dir().unwrap();
        let temp_cwd = tempdir().unwrap();
        let cwd_path = temp_cwd.path();
        env::set_current_dir(cwd_path).unwrap();

        fs::remove_dir(cwd_path).unwrap();

        let result = Config::new();
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::NotFound));

        env::set_current_dir(original_cwd).unwrap();
    }
}
