# `zxc` - directory-oriented command runner

`zxc` is a directory-oriented command runner.
This means it is able to run commands defined in a YAML file.

## Basics of operation

### Definition files location

Two file names are currently supported: `zxc.yml` and `.zxc.yml`.
Those are expected to be located in current working directory.

### Definition file

Following fields are used to define a command:

- command name is used as a key
- `command` - command to run
- `description` - optional command description
- `arguments` - optional arguments

Following fields are used to define an argument:

- argument name is used as a key
- `description` - optional argument description
- `default` - optional default value - if not defined, then argument is required
- `long` - optional long version of parameter, argument name is used by default
- `short` - optional short version of parameter, must be single character long

### Argument substitution

Jinja is used as a template engine.
`command` field can contain double curly braces to provide arguments to the command.

### Example

Simple definition file:

```yaml
greet:
  command: echo "Hello {{ name }}!"
  description: Greets specified person.
  arguments:
    name:
      description: |-
        Name to greet.
      default: User
      long: name
      short: n
```

Use following command to print `Hello John!`:

```bash
zxc greet --name John
```
