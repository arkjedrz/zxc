use std::collections::BTreeMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use serde::Deserialize;

/// YAML-faced definition of an argument.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct YamlArgumentDef {
    /// Flags.
    pub flags: Vec<String>,
    /// Default value.
    pub default: Option<String>,
    /// Description of an argument.
    pub description: Option<String>,
}

/// YAML-faced definition of a command.
#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct YamlCommandDef {
    /// Shell command.
    pub command: String,
    /// Command description.
    pub description: Option<String>,
    /// Arguments used by the command.
    pub arguments: Option<BTreeMap<String, YamlArgumentDef>>,
}

/// Available command definitions.
pub type YamlCommandDefs = BTreeMap<String, YamlCommandDef>;

/// Load YAML command definitions from a string.
pub fn load_yaml_defs_from_str(definition_file_content: &str) -> Result<YamlCommandDefs, Error> {
    match serde_yaml::from_str(definition_file_content) {
        Ok(commands) => Ok(commands),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}

/// Load command definitions from a file.
pub fn load_yaml_defs_from_file(definition_file_path: PathBuf) -> Result<YamlCommandDefs, Error> {
    let definition_file_content = fs::read_to_string(definition_file_path)?;
    load_yaml_defs_from_str(&definition_file_content)
}

/// Merge YAML command definitions.
pub fn merge_yaml_defs(command_defs: Vec<YamlCommandDefs>) -> YamlCommandDefs {
    let mut merged_defs = YamlCommandDefs::new();
    for mut def in command_defs {
        merged_defs.append(&mut def);
    }
    merged_defs
}

#[cfg(test)]
mod load_yaml_defs_from_str_tests {
    use std::io::ErrorKind;

    use super::load_yaml_defs_from_str;

    #[test]
    fn valid_yaml() {
        let yaml_content = r#"
          run:
            command: cargo run -- {{ parameters }}
            description: Run the project
            arguments:
              parameters:
                flags:
                  - --parameters
                description: App parameters.
                default: ""
          test:
            command: cargo test
            description: Run tests
        "#;

        let result = load_yaml_defs_from_str(yaml_content);
        assert!(result.is_ok());

        let defs = result.unwrap();
        assert!(defs.contains_key("run"));
        assert!(defs.contains_key("test"));

        let run_def = defs.get("run").unwrap();
        assert_eq!(run_def.command, "cargo run -- {{ parameters }}");
        assert!(run_def.arguments.is_some());

        let test_def = defs.get("test").unwrap();
        assert_eq!(test_def.command, "cargo test");
        assert!(test_def.arguments.is_none());
    }

    #[test]
    fn invalid_yaml() {
        let yaml_content = r#"
          invalid_yaml: - "test
        "#;

        let result = load_yaml_defs_from_str(yaml_content);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn missing_mandatory_field() {
        let yaml_content = r#"
          name:
            description: Some description.
        "#;
        let result = load_yaml_defs_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }

    #[test]
    fn unknown_command_field() {
        let yaml_content = r#"
          name:
            command: echo "Hello"
            some_unknown_field: asdf
        "#;
        let result = load_yaml_defs_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }

    #[test]
    fn unknown_argument_field() {
        let yaml_content = r#"
          name:
            command: echo "Hello"
            arguments:
              arg1:
                some_unknown_field: asdf
        "#;
        let result = load_yaml_defs_from_str(yaml_content);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::InvalidData));
    }
}

#[cfg(test)]
mod load_yaml_defs_from_file_tests {
    use super::load_yaml_defs_from_file;
    use std::fs::File;
    use std::io::{ErrorKind, Write};
    use std::path::PathBuf;

    use tempfile::tempdir;

    #[test]
    fn valid_file() {
        let yaml_content = r#"
        run:
          command: cargo run
          description: Run the project
        "#;

        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("commands.yaml");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(yaml_content.as_bytes()).unwrap();

        let result = load_yaml_defs_from_file(file_path);
        assert!(result.is_ok());

        let defs = result.unwrap();
        assert!(defs.contains_key("run"));
        assert_eq!(defs.get("run").unwrap().command, "cargo run");
    }

    #[test]
    fn file_missing() {
        let missing_file_path = PathBuf::from("non_existent.yaml");

        let result = load_yaml_defs_from_file(missing_file_path);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::NotFound));
    }
}

#[cfg(test)]
mod merge_yaml_defs_tests {
    use super::merge_yaml_defs;
    use crate::yaml_command_defs::{YamlCommandDef, YamlCommandDefs};

    #[test]
    fn valid_defs() {
        let mut defs1 = YamlCommandDefs::new();
        defs1.insert(
            "build".to_string(),
            YamlCommandDef {
                command: "cargo build".to_string(),
                description: Some("Build the project".to_string()),
                arguments: None,
            },
        );

        let mut defs2 = YamlCommandDefs::new();
        defs2.insert(
            "test".to_string(),
            YamlCommandDef {
                command: "cargo test".to_string(),
                description: Some("Run tests".to_string()),
                arguments: None,
            },
        );

        let merged_defs = merge_yaml_defs(vec![defs1.clone(), defs2.clone()]);
        assert_eq!(merged_defs.len(), 2);
        assert!(merged_defs.contains_key("build"));
        assert!(merged_defs.contains_key("test"));

        assert_eq!(defs1.len(), 1);
        assert_eq!(defs2.len(), 1);
    }

    #[test]
    fn command_overwrite() {
        let mut defs1 = YamlCommandDefs::new();
        defs1.insert(
            "run".to_string(),
            YamlCommandDef {
                command: "cargo run".to_string(),
                description: Some("Run the project".to_string()),
                arguments: None,
            },
        );

        let mut defs2 = YamlCommandDefs::new();
        defs2.insert(
            "run".to_string(),
            YamlCommandDef {
                command: "custom run".to_string(),
                description: Some("Custom run command".to_string()),
                arguments: None,
            },
        );

        let merged_defs = merge_yaml_defs(vec![defs1, defs2]);
        assert_eq!(merged_defs.len(), 1);
        assert_eq!(merged_defs.get("run").unwrap().command, "custom run");
    }

    #[test]
    fn empty_input() {
        let empty_defs = vec![];
        let merged_defs = merge_yaml_defs(empty_defs);
        assert!(merged_defs.is_empty());
    }
}
