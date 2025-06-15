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
    const ALLOWED_NAMES: [&str; 4] = [".zxc.yml", ".zxc.yaml", "zxc.yml", "zxc.yaml"];
    let mut found_files = vec![];
    for name in ALLOWED_NAMES {
        let file_path = directory_path.join(name);
        if file_path.exists() {
            found_files.push(file_path);
        }
    }

    // Disallow multiple found.
    if found_files.len() > 1 {
        return Err(Error::other("Multiple definition files found"));
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

#[cfg(test)]
mod find_def_file_tests {
    use super::find_def_file;
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn file_not_found() {
        let dir = tempdir().unwrap();
        let result = find_def_file(dir.path());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn file_found() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".zxc.yml");
        fs::File::create(&file_path).unwrap();

        let result = find_def_file(dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(file_path));
    }

    #[test]
    fn multiple_file_found() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join(".zxc.yml");
        let file2 = dir.path().join("zxc.yaml");
        fs::File::create(&file1).unwrap();
        fs::File::create(&file2).unwrap();

        let result = find_def_file(dir.path());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::Other);
    }

    #[test]
    fn invalid_path() {
        let invalid_path = Path::new("/invalid/path");
        let result = find_def_file(invalid_path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }
}

#[cfg(test)]
mod get_external_dir_tests {
    use super::get_external_dir;
    use crate::Config;
    use std::path::PathBuf;

    #[test]
    fn ok() {
        let cwd = PathBuf::from("/opt/app");
        let app_home = PathBuf::from("/home/user/.zxc");
        let config = Config { cwd, app_home };

        let expected_path = PathBuf::from("/home/user/.zxc/opt/app");
        let result = get_external_dir(&config);
        assert_eq!(result, expected_path);
    }
}

#[cfg(test)]
mod find_definition_files_tests {
    use super::{find_definition_files, get_external_dir};
    use crate::Config;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn local_and_external() {
        let dir = tempdir().unwrap();
        let cwd = dir.path();
        let app_home = tempdir().unwrap();
        let config = Config {
            cwd: cwd.to_path_buf(),
            app_home: app_home.path().to_path_buf(),
        };

        // Create a local definition file
        let local_def = cwd.join(".zxc.yml");
        let mut local_file = fs::File::create(&local_def).unwrap();
        writeln!(local_file, "local file content").unwrap();

        // Create an external definition file
        let external_dir = get_external_dir(&config);
        fs::create_dir_all(&external_dir).unwrap();
        let external_def = external_dir.join("zxc.yaml");
        let mut external_file = fs::File::create(&external_def).unwrap();
        writeln!(external_file, "external file content").unwrap();

        let result = find_definition_files(&config);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&local_def));
        assert!(result.contains(&external_def));
    }

    #[test]
    fn no_files() {
        let dir = tempdir().unwrap();
        let cwd = dir.path().to_path_buf();
        let app_home = tempdir().unwrap().path().to_path_buf();
        let config = Config { cwd, app_home };

        let result = find_definition_files(&config);
        assert!(result.is_empty());
    }

    #[test]
    fn multiple_local_files() {
        let dir = tempdir().unwrap();
        let cwd = dir.path();
        let app_home = tempdir().unwrap().path().to_path_buf();
        let config = Config {
            cwd: cwd.to_path_buf(),
            app_home,
        };

        // Create multiple local definition files
        fs::File::create(cwd.join(".zxc.yml")).unwrap();
        fs::File::create(cwd.join("zxc.yaml")).unwrap();

        let result = std::panic::catch_unwind(|| find_definition_files(&config));
        assert!(result.is_err());
    }
}
