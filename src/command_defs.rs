use std::collections::BTreeMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use serde::Deserialize;

/// Definition of an argument.
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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

/// Load command definitions from a string.
pub fn load_definitions_from_str(definition_file_content: &str) -> Result<CommandDefs, Error> {
    match serde_yaml::from_str(definition_file_content) {
        Ok(commands) => Ok(commands),
        Err(_) => Err(Error::from(ErrorKind::InvalidData)),
    }
}

/// Load command definitions from a file.
pub fn load_definitions_from_file(definition_file_path: PathBuf) -> Result<CommandDefs, Error> {
    let definition_file_content = match fs::read_to_string(definition_file_path) {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    load_definitions_from_str(&definition_file_content)
}

pub fn merge_definitions(command_defs: Vec<CommandDefs>) -> CommandDefs {
    let mut merged_defs = CommandDefs::new();
    for mut def in command_defs {
        merged_defs.append(&mut def);
    }
    merged_defs
}

#[cfg(test)]
mod load_definitions_from_file_tests {
    use std::io::ErrorKind;

    use super::load_definitions_from_str;

    #[test]
    fn all_params() {
        let name = "greet";
        let arg1_name = "name";
        let arg1_desc = "Name.";
        let arg1_default = "John";
        let arg1_long = "name";
        let arg1_short = "n";
        let arg2_name = "surname";
        let arg2_desc = "Surname.";
        let arg2_default = "Smith";
        let arg2_long = "surname";
        let arg2_short = "s";
        let command = format!("echo \"Hello {{ {arg1_name} }} {{ {arg2_name} }}\"");
        let description = "Say hello.";

        let yaml_content = format!(
            "{name}:
  command: {command}
  description: {description}
  arguments:
    {arg1_name}:
      description: |-
        {arg1_desc}
      default: {arg1_default}
      long: {arg1_long}
      short: {arg1_short}
    {arg2_name}:
      description: {arg2_desc}
      default: {arg2_default}
      long: {arg2_long}
      short: {arg2_short}"
        );
        let command_defs = load_definitions_from_str(yaml_content.as_str()).unwrap();
        assert!(command_defs.len() == 1);

        let command_def = command_defs.get(name).unwrap();
        assert_eq!(command_def.command, command);
        assert_eq!(command_def.description.as_ref().unwrap(), description);

        let args = command_def.arguments.clone().unwrap();
        assert!(args.len() == 2);

        let arg1 = args.get(arg1_name).unwrap();
        assert_eq!(arg1.description.as_ref().unwrap(), arg1_desc);
        assert_eq!(arg1.default.as_ref().unwrap(), arg1_default);
        assert_eq!(arg1.long.as_ref().unwrap(), arg1_long);
        assert_eq!(arg1.short.as_ref().unwrap(), arg1_short);

        let arg2 = args.get(arg2_name).unwrap();
        assert_eq!(arg2.description.as_ref().unwrap(), arg2_desc);
        assert_eq!(arg2.default.as_ref().unwrap(), arg2_default);
        assert_eq!(arg2.long.as_ref().unwrap(), arg2_long);
        assert_eq!(arg2.short.as_ref().unwrap(), arg2_short);
    }

    #[test]
    fn no_optionals() {
        let name = "hello_world";
        let command = "echo \"Hello\"";
        let yaml_content = format!(
            "{name}:
  command: {command}"
        );

        let command_defs = load_definitions_from_str(yaml_content.as_str()).unwrap();
        assert!(command_defs.len() == 1);

        let command_def = command_defs.get(name).unwrap();
        assert_eq!(command_def.command, command);
        assert!(command_def.description.is_none());
        assert!(command_def.arguments.is_none());
    }

    #[test]
    fn missing_mandatory_field() {
        let yaml_content = "name:
  description: description
  arguments:
    arg1_name:
      description: |-
        arg1_desc
      default: arg1_default
      long: arg1_long
      short: arg1_short
    arg2_name:
      description: arg2_desc
      default: arg2_default
      long: arg2_long
      short: arg2_short
";
        let result = load_definitions_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }

    #[test]
    fn unknown_command_field() {
        let yaml_content = "name:
  command: command
  description: description
  some_unknown_field: asdf
  arguments:
    arg1_name:
      description: |-
        arg1_desc
      default: arg1_default
      long: arg1_long
      short: arg1_short
    arg2_name:
      description: arg2_desc
      default: arg2_default
      long: arg2_long
      short: arg2_short
";
        let result = load_definitions_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }

    #[test]
    fn unknown_argument_field() {
        let yaml_content = "name:
  command: command
  description: description
  arguments:
    arg1_name:
      description: |-
        arg1_desc
      default: arg1_default
      long: arg1_long
      short: arg1_short
      some_unknown_field: asdf
    arg2_name:
      description: arg2_desc
      default: arg2_default
      long: arg2_long
      short: arg2_short
";
        let result = load_definitions_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }

    #[test]
    fn malformed_yaml() {
        let yaml_content = "name:
      command: command
        description: description
    ";
        let result = load_definitions_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }
}

#[cfg(test)]
mod merge_definitions_tests {
    use super::{load_definitions_from_str, merge_definitions};

    #[test]
    fn valid() {
        let content1 = "first_command:
  command: echo \"Hello\"
      ";
        let defs1 = load_definitions_from_str(content1).unwrap();

        let content2 = "second_command:
  command: echo \"world\"";
        let defs2 = load_definitions_from_str(content2).unwrap();

        let defs_vec = vec![defs1, defs2];
        let merged = merge_definitions(defs_vec);

        assert!(merged.len() == 2);
        assert!(merged
            .get("first_command")
            .is_some_and(|v| v.command == "echo \"Hello\""));
        assert!(merged
            .get("second_command")
            .is_some_and(|v| v.command == "echo \"world\""));
    }

    #[test]
    fn command_overwrite() {
        let content1 = "same_command_name:
  command: echo \"Hello\"
      ";
        let defs1 = load_definitions_from_str(content1).unwrap();

        let content2 = "same_command_name:
  command: echo \"world\"";
        let defs2 = load_definitions_from_str(content2).unwrap();

        let defs_vec = vec![defs1, defs2];
        let merged = merge_definitions(defs_vec);

        assert!(merged.len() == 1);
        assert!(merged
            .get("same_command_name")
            .is_some_and(|v| v.command == "echo \"world\""));
    }

    #[test]
    fn empty_input() {
        let empty_defs = vec![];
        let merged_defs = merge_definitions(empty_defs);
        assert!(merged_defs.is_empty());
    }
}
