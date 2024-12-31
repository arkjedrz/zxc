use minijinja::{Environment, Error, UndefinedBehavior, Value};
use std::collections::BTreeMap;

/// Resolve shell command.
pub fn resolve_command(
    unresolved_command: &str,
    arguments: &BTreeMap<String, String>,
) -> Result<String, Error> {
    // Create environment and add command template.
    const ENV_NAME: &str = "command";
    let mut env = Environment::new();
    env.set_undefined_behavior(UndefinedBehavior::Strict);
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

#[cfg(test)]
mod resolve_command_tests {
    use std::collections::BTreeMap;

    use minijinja::ErrorKind;

    use super::resolve_command;

    #[test]
    fn valid() {
        let unresolved = "Hello \"{{ name }}\"";
        let arguments = BTreeMap::from([("name".to_string(), "John".to_string())]);

        let resolved = resolve_command(unresolved, &arguments).unwrap();
        let expected = "Hello \"John\"";
        assert_eq!(resolved, expected);
    }

    #[test]
    fn empty_input() {
        let unresolved = "";
        let arguments = BTreeMap::new();

        let resolved = resolve_command(unresolved, &arguments).unwrap();
        let expected = "";
        assert_eq!(resolved, expected);
    }

    #[test]
    fn invalid_syntax() {
        let unresolved = "Hello \"{{{ name }}\"";
        let arguments = BTreeMap::from([("name".to_string(), "John".to_string())]);

        let result = resolve_command(unresolved, &arguments);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::SyntaxError));
    }

    #[test]
    fn too_few_args() {
        let unresolved = "Hello \"{{ name }}\"";
        let arguments = BTreeMap::new();

        let result = resolve_command(unresolved, &arguments);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::UndefinedError));
    }
}
