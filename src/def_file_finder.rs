use crate::config::Config;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// Find definition file in specified directory.
fn find_def_file(directory_path: &PathBuf) -> Result<Option<PathBuf>, Error> {
    // Check path exists.
    if !directory_path.exists() {
        return Err(Error::from(ErrorKind::NotFound));
    }

    // Check path is a directory.
    if !directory_path.is_dir() {
        return Err(Error::from(ErrorKind::NotADirectory));
    }

    // Iterate through allowed file names.
    const ALLOWED_NAMES: [&str; 5] = [".zxc.yml", ".zxc.yaml", ".zxc", "zxc.yml", "zxc.yaml"];
    let mut found_files = vec![];
    for name in ALLOWED_NAMES {
        let file_path = directory_path.join(name);
        if file_path.exists() {
            found_files.push(file_path);
        }
    }

    // Disallow multiple found.
    if found_files.len() > 1 {
        return Err(Error::new(
            ErrorKind::Other,
            "Multiple definition files found",
        ));
    }

    // Check if any found.
    if found_files.is_empty() {
        return Ok(None);
    }

    Ok(Some(found_files[0].clone()))
}

/// Get external directory for CWD.
/// CWD path is encoded to base64.
///
/// Returns path to `$HOME/.zxc/<base64-encoded CWD path>`.
pub fn get_external_dir(config: &Config) -> PathBuf {
    let cwd_str = config
        .cwd
        .to_str()
        .expect("Failed to convert path to string");
    let cwd_encoded = STANDARD.encode(cwd_str);
    config.app_home.join(cwd_encoded)
}

/// Find definition files.
/// - local - from CWD.
/// - external - from `$HOME/.zxc/<base64-encoded CWD path>`.
///
/// Following file names are allowed:
/// - `.zxc.yml`
/// - `.zxc.yaml`
/// - `.zxc`
/// - `zxc.yml`
/// - `zxc.yaml`
///
/// Only one such file is allowed in a directory.
///
/// Returns empty container if nothing is found.
pub fn find_definition_files(config: &Config) -> Vec<PathBuf> {
    let mut found_files: Vec<PathBuf> = vec![];

    // Get local definition file.
    let local_def_file = find_def_file(&config.cwd).expect("Failed to find local definition file");
    match local_def_file {
        Some(path) => found_files.push(path),
        None => (),
    }

    // Get external dir path.
    let external_dir = get_external_dir(&config);
    if external_dir.exists() && external_dir.is_dir() {
        let external_def_file =
            find_def_file(&external_dir).expect("Failed to find external definition file");
        match external_def_file {
            Some(path) => found_files.push(path),
            None => (),
        }
    }
    found_files
}
