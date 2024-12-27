use std::path::PathBuf;

/// Find definition files.
/// - From CWD.
/// - TODO: from $HOME.
/// Returns empty container if nothing is found.
pub fn find_definition_files() -> Vec<PathBuf> {
    // Get CWD.
    let cwd = std::env::current_dir().expect("Current working directory not found");

    // List of allowed file names.
    const ALLOWED_NAMES: [&str; 2] = [".zxc.yml", "zxc.yml"];

    // Iterate through allowed names.
    let mut definition_files = vec![];
    for name in ALLOWED_NAMES {
        let file_path = cwd.join(name);
        if file_path.exists() {
            definition_files.push(file_path);
        }
    }
    definition_files
}
