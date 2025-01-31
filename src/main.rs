mod cli;
mod command_resolver;
mod command_runner;
mod config;
mod def_file_finder;
mod yaml_command_defs;
mod zxc_command_defs;

use std::collections::BTreeMap;

use cli::build_cli;
use command_resolver::resolve_command;
use command_runner::run_command;
use config::Config;
use def_file_finder::find_definition_files;
use yaml_command_defs::{load_yaml_defs_from_file, merge_yaml_defs};
use zxc_command_defs::{find_zxc_command_def, load_zxc_defs};

fn main() {
    // Initialize and load configuration.
    let config = Config::new().expect("Failed to init");

    // Find definition files.
    let definition_files = find_definition_files(&config);
    if definition_files.is_empty() {
        panic!("No definition files found");
    }

    // Load command data.
    // First load in YAML-faced structure.
    let mut yaml_command_defs_vec = vec![];
    for definition_file_path in definition_files {
        let defs_from_file = load_yaml_defs_from_file(definition_file_path).unwrap();
        yaml_command_defs_vec.push(defs_from_file);
    }
    let yaml_command_defs = merge_yaml_defs(yaml_command_defs_vec);

    // Then transform to app-faced structure
    let zxc_command_defs = match load_zxc_defs(yaml_command_defs) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    // Build CLI and parse arguments.
    let cli_command = build_cli(&zxc_command_defs);
    let cli_args = cli_command.get_matches();

    if let Some((subcommand_name, subcommand_args)) = cli_args.subcommand() {
        // Find command definition.
        let zxc_command_def = find_zxc_command_def(zxc_command_defs, subcommand_name).unwrap();

        // Process required command data.
        let unresolved_command = &zxc_command_def.command;
        let mut arguments: BTreeMap<String, String> = BTreeMap::new();
        for id in subcommand_args.ids() {
            let value: &String = subcommand_args.get_one(id.as_str()).unwrap();
            arguments.insert(id.to_string(), value.to_string());
        }

        // Resolve command.
        let resolved_command = resolve_command(unresolved_command, &arguments).unwrap();

        // Run command.
        let run_status = run_command(&resolved_command);
        let exit_code = run_status
            .unwrap()
            .code()
            .expect("Process terminated by signal");
        std::process::exit(exit_code);
    }
}
