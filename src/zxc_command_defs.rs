use std::io::{Error, ErrorKind};

use crate::yaml_command_defs::YamlCommandDefs;

/// Flag type.
#[derive(Clone, Debug)]
pub enum FlagType {
    /// Short flag. Always starts with '-'. E.g., '-n'.
    Short(String),
    /// Long flag. Always starts with '--'. E.g., '--name'.
    Long(String),
}

/// Argument type.
#[derive(Clone, Debug)]
pub enum ArgumentType {
    /// Named argument ('-n', '--name').
    Named(FlagType),
    /// Positional argument - argument name is used.
    Positional,
}

#[derive(Clone, Debug)]
pub struct ZxcArgumentDef {
    /// Argument name.
    pub name: String,
    /// Flags.
    pub flags: Vec<ArgumentType>,
    /// Default value.
    pub default: Option<String>,
    /// Description of an argument.
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ZxcCommandDef {
    /// Command name.
    pub name: String,
    /// Shell command.
    pub command: String,
    /// Command description.
    pub description: Option<String>,
    /// Arguments used by the command.
    pub arguments: Vec<ZxcArgumentDef>,
}

pub type ZxcCommandDefs = Vec<ZxcCommandDef>;

/// Change representation from YAML-faced to app-faced.
pub fn load_zxc_defs(yaml_command_defs: YamlCommandDefs) -> Result<ZxcCommandDefs, Error> {
    let mut zxc_command_defs = Vec::new();
    for (yaml_command_name, yaml_command_def) in yaml_command_defs {
        // Prepare basic parameters.
        let name = yaml_command_name;
        let command = yaml_command_def.command;
        let description = yaml_command_def.description;

        // Iterate through arguments.
        let mut arguments = Vec::new();
        if let Some(yaml_arguments) = yaml_command_def.arguments {
            for (yaml_argument_name, yaml_argument_def) in yaml_arguments {
                // Prepare basic parameters.
                let name = yaml_argument_name;
                let default = yaml_argument_def.default;
                let description = yaml_argument_def.description;

                // Iterate through flags.
                let mut flags = Vec::new();
                for yaml_flag in yaml_argument_def.flags {
                    let yaml_flag_str = yaml_flag.as_str();
                    // Handle long option flags.
                    if yaml_flag_str.starts_with("--") {
                        let zxc_flag = match yaml_flag_str.strip_prefix("--") {
                            Some(x) => ArgumentType::Named(FlagType::Long(x.to_string())),
                            None => {
                                let error_message =
                                    format!("Failed to strip '--' prefix from: {yaml_flag_str}");
                                return Err(Error::new(ErrorKind::InvalidData, error_message));
                            }
                        };
                        flags.push(zxc_flag);
                    }
                    // Handle short option flags.
                    else if yaml_flag_str.starts_with("-") {
                        if yaml_flag_str.len() != 2 {
                            let error_message = format!(
                                "Short flag must consist of two characters: {yaml_flag_str}"
                            );
                            return Err(Error::new(ErrorKind::InvalidData, error_message));
                        }

                        let zxc_flag = match yaml_flag_str.strip_prefix("-") {
                            Some(x) => ArgumentType::Named(FlagType::Short(x.to_string())),
                            None => {
                                let error_message =
                                    format!("Failed to strip '-' prefix from: {yaml_flag_str}");
                                return Err(Error::new(ErrorKind::InvalidData, error_message));
                            }
                        };
                        flags.push(zxc_flag);
                    }
                    // Handle positional arguments.
                    else {
                        let zxc_flag = ArgumentType::Positional;
                        flags.push(zxc_flag)
                    }
                }

                // Prevent empty flags.
                if flags.is_empty() {
                    let error_message = "No flags are defined";
                    return Err(Error::new(ErrorKind::InvalidData, error_message));
                }

                // Prevent more than two flags.
                if flags.len() > 2 {
                    let error_message = format!("More than two flags defined: {:#?}", &flags);
                    return Err(Error::new(ErrorKind::InvalidData, error_message));
                }

                // Count argument and flag types occurences.
                let mut num_short = 0;
                let mut num_long = 0;
                let mut num_positional = 0;
                for flag in &flags {
                    match flag {
                        ArgumentType::Named(flag_type) => match flag_type {
                            FlagType::Short(_) => num_short += 1,
                            FlagType::Long(_) => num_long += 1,
                        },
                        ArgumentType::Positional => num_positional += 1,
                    }
                }

                // Prevent both positional and named argument flags.
                if num_positional > 0 && (num_short > 0 || num_long > 0) {
                    let error_message = format!(
                        "Both positional and named argument flags are defined: {:#?}",
                        &flags
                    );
                    return Err(Error::new(ErrorKind::InvalidData, error_message));
                }

                // Prevent multiple positional argument flags.
                if num_positional > 1 {
                    let error_message =
                        format!("Multiple positional arguments are defined: {:#?}", &flags);
                    return Err(Error::new(ErrorKind::InvalidData, error_message));
                }

                // Prevent multiple short flags.
                if num_short > 1 {
                    let error_message = format!(
                        "Multiple named argument short flags are defined: {:#?}",
                        &flags
                    );
                    return Err(Error::new(ErrorKind::InvalidData, error_message));
                }

                // Prevent multiple long flags.
                if num_long > 1 {
                    let error_message = format!(
                        "Multiple named argument long flags are defined: {:#?}",
                        &flags
                    );
                    return Err(Error::new(ErrorKind::InvalidData, error_message));
                }

                arguments.push(ZxcArgumentDef {
                    name,
                    flags,
                    default,
                    description,
                });
            }
        }

        zxc_command_defs.push(ZxcCommandDef {
            name,
            command,
            description,
            arguments,
        });
    }

    Ok(zxc_command_defs)
}

/// Find command definition using name.
pub fn find_zxc_command_def(
    zxc_command_defs: ZxcCommandDefs,
    command_name: &str,
) -> Option<ZxcCommandDef> {
    zxc_command_defs
        .into_iter()
        .find(|x| x.name == command_name)
}
