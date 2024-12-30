use minijinja::{Environment, Error, Value};
use std::collections::BTreeMap;

/// Resolve shell command.
pub fn resolve_command(
    unresolved_command: &str,
    arguments: &BTreeMap<String, String>,
) -> Result<String, Error> {
    // Create environment and add command template.
    const ENV_NAME: &str = "command";
    let mut env = Environment::new();
    match env.add_template(ENV_NAME, unresolved_command) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    // Get command template.
    let template = match env.get_template(ENV_NAME) {
        Ok(template) => template,
        Err(e) => return Err(e),
    };

    // Render output string based on provided arguments.
    let ctx = Value::from_iter(arguments.iter());
    template.render(ctx)
}
