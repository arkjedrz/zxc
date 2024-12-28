mod cli;
mod command_defs;
mod command_resolver;
mod command_runner;
mod config;
mod def_file_finder;

use std::collections::BTreeMap;

use cli::build_cli;
use command_defs::load_command_definitions;
use command_resolver::resolve_command;
use command_runner::run_command;
use config::Config;
use def_file_finder::{find_definition_files, get_external_dir};

fn main() {
    // Initialize and load configuration.
    let config = Config::new().expect("Failed to init");

    // Find definition files.
    let definition_files = find_definition_files(&config);
    if definition_files.is_empty() {
        panic!("No definition files found");
    }

    // Load command data.
    let loaded_commands = load_command_definitions(definition_files);

    // Build CLI and parse arguments.
    let cli_command = build_cli(&loaded_commands);
    let cli_args = cli_command.get_matches();

    match cli_args.subcommand() {
        Some((subcommand_name, _)) if subcommand_name == "external_dir" => {
            let external_dir = get_external_dir(&config);
            let external_dir_str = external_dir
                .to_str()
                .expect("Failed to convert path to string");
            println!("{}", external_dir_str);
        }
        Some((subcommand_name, subcommand_args)) => {
            // Get command definition.
            let command_def = loaded_commands.get(subcommand_name).unwrap();

            // Process required command data.
            let unresolved_command = &command_def.command;
            let mut arguments: BTreeMap<String, String> = BTreeMap::new();
            for id in subcommand_args.ids() {
                let value: &String = subcommand_args.get_one(id.as_str()).unwrap();
                arguments.insert(id.to_string(), value.to_string());
            }

            // Resolve command.
            let resolved_command = resolve_command(&unresolved_command, &arguments).unwrap();

            // Run command.
            let run_status = run_command(&resolved_command);
            let exit_code = run_status
                .unwrap()
                .code()
                .expect("Process terminated by signal");
            std::process::exit(exit_code);
        }
        None => (),
    }
}
