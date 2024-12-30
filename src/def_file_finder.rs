use crate::config::Config;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Find definition file in specified directory.
fn find_def_file(directory_path: &Path) -> Result<Option<PathBuf>, Error> {
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
    // It's not an error that nothing was found.
    if found_files.is_empty() {
        return Ok(None);
    }

    Ok(Some(found_files[0].clone()))
}

/// Returns path to `$HOME/.zxc/<mirrored CWD path>`.
fn get_external_dir(config: &Config) -> PathBuf {
    let mut app_home = config.app_home.clone();
    let cwd = config.cwd.as_os_str();

    let external_dir = app_home.as_mut_os_string();
    external_dir.push(cwd);

    PathBuf::from(external_dir.as_os_str())
}

/// Find definition files.
/// - local - from CWD.
/// - external - from `$HOME/.zxc/<mirrored CWD path>`.
///   E.g., if CWD is `/opt/app/` then `$HOME/.zxc/opt/app/` should be used.
///
/// Following file names are allowed:
/// - `.zxc.yml`
/// - `.zxc.yaml`
/// - `.zxc`
/// - `zxc.yml`
/// - `zxc.yaml`
///
/// It's not allowed to have multiple definition files in one directory.
///
/// Returns empty container if nothing is found.
pub fn find_definition_files(config: &Config) -> Vec<PathBuf> {
    let mut found_files: Vec<PathBuf> = vec![];

    // Get local definition file.
    let local_def_file = find_def_file(&config.cwd).expect("Failed to find local definition file");
    if let Some(path) = local_def_file {
        found_files.push(path);
    }

    // Get external dir path.
    let external_dir = get_external_dir(config);
    if external_dir.exists() && external_dir.is_dir() {
        let external_def_file =
            find_def_file(&external_dir).expect("Failed to find external definition file");
        if let Some(path) = external_def_file {
            found_files.push(path);
        }
    }
    found_files
}
