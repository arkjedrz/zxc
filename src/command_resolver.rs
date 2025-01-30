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
    let template = env.get_template(ENV_NAME)?;

    // Render output string based on provided arguments.
    let ctx = Value::from_iter(arguments.iter());
    template.render(ctx)
}

#[cfg(test)]
mod resolve_command_tests {
    use std::collections::BTreeMap;

    use super::resolve_command;

    use minijinja::ErrorKind;

    #[test]
    fn valid_arguments() {
        let unresolved_command = "echo {{ arg1 }} {{ arg2 }}";
        let mut arguments = BTreeMap::new();
        arguments.insert("arg1".to_string(), "hello".to_string());
        arguments.insert("arg2".to_string(), "world".to_string());

        let result = resolve_command(unresolved_command, &arguments);
        assert!(result.is_ok_and(|v| v == "echo hello world"));
    }

    #[test]
    fn missing_arguments() {
        let unresolved_command = "echo {{ arg1 }} {{ arg2 }}";
        let mut arguments = BTreeMap::new();
        arguments.insert("arg1".to_string(), "hello".to_string());

        let result = resolve_command(unresolved_command, &arguments);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::UndefinedError));
    }

    #[test]
    fn empty_arguments() {
        let unresolved_command = "echo {{ arg1 }}";
        let arguments = BTreeMap::new();

        let result = resolve_command(unresolved_command, &arguments);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::UndefinedError));
    }

    #[test]
    fn no_placeholders() {
        let unresolved_command = "echo hello world";
        let arguments = BTreeMap::new();

        let result = resolve_command(unresolved_command, &arguments);
        assert!(result.is_ok_and(|v| v == "echo hello world"));
    }

    #[test]
    fn invalid_template_syntax() {
        let unresolved_command = "echo {{ arg1 {{ arg2 }}";
        let arguments = BTreeMap::new();

        let result = resolve_command(unresolved_command, &arguments);
        assert!(result.is_err_and(|e| e.kind() == ErrorKind::SyntaxError));
    }

    #[test]
    fn extra_arguments() {
        let unresolved_command = "echo {{ arg1 }}";
        let mut arguments = BTreeMap::new();
        arguments.insert("arg1".to_string(), "hello".to_string());
        arguments.insert("arg2".to_string(), "world".to_string());

        let result = resolve_command(unresolved_command, &arguments);
        assert!(result.is_ok_and(|v| v == "echo hello"));
    }
}
