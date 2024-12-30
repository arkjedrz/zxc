use shlex::split;
use std::io::Error;
use std::process::{Command, ExitStatus};

/// Spawn new process based on provided resolved command string.
pub fn run_command(command: &str) -> Result<ExitStatus, Error> {
    // Split shell command.
    let split_result = split(command).expect("Failed to split command");
    // Split command name and arguments.
    // TODO: support environment variables.
    let command = &split_result[0];
    let args = &split_result[1..];

    // Spawn process.
    let spawn_result = Command::new(command).args(args).spawn();
    let mut child = match spawn_result {
        Ok(child) => child,
        Err(e) => return Err(e),
    };

    // Wait for process to finish.
    child.wait()
}
