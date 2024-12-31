use run_script::types::ScriptError::{Description, FsIOError, IOError};
use run_script::{spawn, IoOptions, ScriptError, ScriptOptions};
use std::io::{Error, ErrorKind};
use std::process::ExitStatus;

/// Convert all errors to `std::io::Error`.
fn match_error(script_error: ScriptError) -> Error {
    match script_error {
        IOError(io_error) => io_error,
        FsIOError(fsio_error) => Error::new(ErrorKind::Other, fsio_error.to_string()),
        Description(description) => Error::new(ErrorKind::Other, description),
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
