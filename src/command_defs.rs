use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

/// Definition of an argument.
#[derive(Deserialize, Clone)]
pub struct ArgumentDef {
    /// Description.
    pub description: Option<String>,
    /// Default value.
    pub default: Option<String>,
    /// Long version of parameter.
    /// Argument name is used by default.
    pub long: Option<String>,
    /// Short version of parameter.
    /// Must be single character long.
    pub short: Option<String>,
}

/// Definition of a command.
#[derive(Deserialize, Clone)]
pub struct CommandDef {
    /// Shell command.
    pub command: String,
    /// Command description.
    pub description: Option<String>,
    /// Arguments used by the command.
    pub arguments: Option<BTreeMap<String, ArgumentDef>>,
}

/// Available command definitions.
pub type CommandDefs = BTreeMap<String, CommandDef>;

/// Load command definitions.
/// Returns empty container if nothing is found.
pub fn load_command_definitions(definition_files: Vec<PathBuf>) -> CommandDefs {
    let mut merged_commands = CommandDefs::new();
    for file_path in definition_files {
        // Read file content.
        let content = fs::read_to_string(file_path).expect("Unable to read the file");

        // Parse YAML file.
        let commands: &mut CommandDefs =
            &mut serde_yaml::from_str(content.as_str()).expect("Unable to parse file content");

        // TODO: raise an error on key being overwritten.
        merged_commands.append(commands);
    }

    merged_commands
}
