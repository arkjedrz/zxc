use crate::zxc_command_defs::{ArgumentType, FlagType, ZxcCommandDef, ZxcCommandDefs};
use clap::{command, Arg, Command};

/// Build `Command` object for a subcommand.
fn build_subcommand(zxc_command_def: &ZxcCommandDef) -> Command {
    let mut cli_command = Command::new(zxc_command_def.name.clone());

    // Add 'about' section - containing description and command.
    let mut about_string = String::new();
    if let Some(description) = &zxc_command_def.description {
        let formatted = format!("{description}\n\n");
        about_string.push_str(&formatted);
    }

    let formatted_shell_command = format!("Command: {}", &zxc_command_def.command);
    about_string.push_str(&formatted_shell_command);

    cli_command = cli_command.about(about_string);

    // Add arguments.
    for zxc_argument_def in &zxc_command_def.arguments {
        let mut cli_argument = Arg::new(zxc_argument_def.name.clone());

        // Add flags.
        for zxc_flag in &zxc_argument_def.flags {
            cli_argument = match zxc_flag {
                ArgumentType::Named(flag_type) => match flag_type {
                    FlagType::Short(x) => {
                        let x = x.as_str().chars().next().unwrap();
                        cli_argument.short(x)
                    }
                    FlagType::Long(x) => cli_argument.long(x),
                },
                // Nothing needs to be done for positional - argument name is used.
                ArgumentType::Positional => cli_argument,
            }
        }

        // Add default value.
        cli_argument = match &zxc_argument_def.default {
            Some(x) => cli_argument.default_value(x),
            None => cli_argument.required(true),
        };

        // Add description.
        cli_argument = match &zxc_argument_def.description {
            Some(x) => cli_argument.help(x),
            None => cli_argument,
        };

        cli_command = cli_command.arg(cli_argument);
    }

    cli_command
}

pub fn build_cli(zxc_command_defs: &ZxcCommandDefs) -> Command {
    // Create base `clap` command.
    let mut main_command = command!().subcommand_required(true);
    // Add subcommands.
    for zxc_command_def in zxc_command_defs {
        let subcommand = build_subcommand(zxc_command_def);
        main_command = main_command.subcommand(subcommand);
    }
    main_command
}
