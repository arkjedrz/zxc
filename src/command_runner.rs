use run_script::types::ScriptError::{Description, FsIOError, IOError};
use run_script::{IoOptions, ScriptError, ScriptOptions, spawn};
use std::io::Error;
use std::process::ExitStatus;

/// Convert all errors to `std::io::Error`.
fn match_error(script_error: ScriptError) -> Error {
    match script_error {
        IOError(io_error) => io_error,
        FsIOError(fsio_error) => Error::other(fsio_error.to_string()),
        Description(description) => Error::other(description),
    }
}

/// Spawn new process based on provided resolved command string.
pub fn run_command(command: &str) -> Result<ExitStatus, Error> {
    // Set parameters.
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.input_redirection = IoOptions::Inherit;
    options.output_redirection = IoOptions::Inherit;

    // Spawn process.
    let spawn_result = spawn(command, &args, &options);
    let mut child = match spawn_result {
        Ok(child) => child,
        Err(e) => return Err(match_error(e)),
    };

    // Wait for process to finish.
    child.wait()
}

#[cfg(test)]
mod match_error_tests {
    use super::match_error;
    use fsio::error::FsIOError;
    use run_script::types::ScriptError;
    use std::io;

    #[test]
    fn io_error() {
        let description = "File not found";
        let io_error = io::Error::new(io::ErrorKind::NotFound, description);
        let script_error = ScriptError::IOError(io_error);
        let result = match_error(script_error);

        assert_eq!(result.kind(), io::ErrorKind::NotFound);
        assert_eq!(result.to_string(), description);
    }

    #[test]
    fn fsio_error() {
        let description = "Not file";
        let fsio_error = FsIOError::NotFile(description.to_string());
        let script_error = ScriptError::FsIOError(fsio_error);
        let result = match_error(script_error);

        assert_eq!(result.kind(), io::ErrorKind::Other);
        assert_eq!(result.to_string(), description);
    }

    #[test]
    fn description() {
        let description = "Script failed";
        let script_error = ScriptError::Description(description);
        let result = match_error(script_error);

        assert_eq!(result.kind(), io::ErrorKind::Other);
        assert_eq!(result.to_string(), description);
    }
}

#[cfg(test)]
mod run_command_tests {
    use super::run_command;
    use std::env::consts::OS;

    #[test]
    fn success() {
        let command = "echo Hello, World!";

        let result = run_command(command);
        assert!(result.is_ok());

        let status = result.unwrap();
        assert!(status.success());
    }

    #[test]
    fn failure() {
        let command = "(exit 1)";

        let result = run_command(command);
        assert!(result.is_ok());

        let status = result.unwrap();
        assert_eq!(status.code().unwrap(), 1);
    }

    #[test]
    fn non_existent_command() {
        let command = "non_existent_command";

        let result = run_command(command);
        assert!(result.is_ok());

        let status = result.unwrap();
        let expected = if OS == "windows" { 1 } else { 127 };
        assert_eq!(status.code().unwrap(), expected);
    }
}
