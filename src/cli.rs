use crate::command_defs;

use clap::{command, Arg, Command};
use command_defs::{CommandDef, CommandDefs};

/// Build `Command` object for a subcommand.
fn build_subcommand(cmd_name: &String, cmd_data: &CommandDef) -> Command {
    let mut cmd = Command::new(cmd_name);

    // Add 'about' section - containing description and command.
    let mut about_string = String::new();
    match &cmd_data.description {
        Some(description) => {
            let formatted = format!("{description}\n\n");
            about_string.push_str(&formatted);
        }
        None => (),
    }

    let formatted_cmd = format!("Command: {}", &cmd_data.command);
    about_string.push_str(&formatted_cmd);

    cmd = cmd.about(about_string);

    // Add arguments.
    cmd = match &cmd_data.arguments {
        Some(arguments) => {
            for (arg_name, arg_data) in arguments {
                let mut arg = Arg::new(arg_name);

                // Add long parameter version.
                arg = match &arg_data.long {
                    Some(long) => arg.long(long),
                    None => arg.long(arg_name),
                };

                // Add short parameter version.
                arg = match &arg_data.short {
                    Some(short) => {
                        if short.len() != 1 {
                            panic!("Short parameter version must be 1 character long");
                        }
                        let first_char = short.chars().next().unwrap();
                        arg.short(first_char)
                    }
                    None => arg,
                };

                // Add description.
                arg = match &arg_data.description {
                    Some(description) => arg.help(description),
                    None => arg,
                };

                // Add default value.
                arg = match &arg_data.default {
                    Some(default) => arg.default_value(default),
                    None => arg.required(true),
                };

                cmd = cmd.arg(arg);
            }
            cmd
        }
        None => cmd,
    };

    cmd
}

fn build_external_dir_command() -> Command {
    // Create command.
    Command::new("external_dir")
        .about("Print path to the external definition file directory for current working directory")
}

pub fn build_cli(commands: &CommandDefs) -> Command {
    // Create base `clap` command.
    let mut main_command = command!().subcommand_required(true);

    // Add `external_dir` subcommand.
    main_command = main_command.subcommand(build_external_dir_command());

    // Add loaded subcommands.
    for (name, data) in commands {
        let subcommand = build_subcommand(name, &data);
        main_command = main_command.subcommand(subcommand);
    }
    main_command
}
