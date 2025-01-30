# `zxc` - directory-oriented command runner

`zxc` is a directory-oriented command runner.
This means it is able to run commands defined by YAML files.

## Pre-commit setup

[pre-commit](https://pre-commit.com/) tool is used.

To install pre-commit hooks:

```bash
pre-commit install
```

To run pre-commit hooks:

```bash
pre-commit run -a
```

## Basics of operation

### Definition files location

Definition files are searched for in two places:

- local - from CWD.
- external - from `$HOME/.zxc/<mirrored CWD path>`.
E.g., if CWD is `/opt/app/` then `$HOME/.zxc/opt/app/` should be used.

Following file names are allowed:

- `.zxc.yml`
- `.zxc.yaml`
- `zxc.yml`
- `zxc.yaml`

It's expected that at least one definition file is found.
It's not allowed to have multiple definition files in one directory.

External definition file takes precedence.
This might cause command to be overwritten if defined in both files.

### Definition file

#### Defining a command

Following fields are used to define a command:

- command name is used as a key
- `command` - shell command to run - mandatory
- `description` - description - optional
- `arguments` - arguments - optional

#### Defining an argument

Following fields are used to define an argument:

- argument name is used as a key
- `flags` - list of flags - mandatory
  - named arguments contain flags starting with `-` and/or `--`:
    - multiple short/long flags cannot be defined for the same argument
    - 1 or 2 flags are expected
  - positional arguments contain flag not starting with `-` or `--`.
  - argument cannot be simultanously named and positional
- `default` - default value - optional
  - argument is considered required if default value is not specified
- `description` - description - optional

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
      flags: ["-n", "--name"]
      default: User
      description: |-
        Name to greet.
```

Use following command to print `Hello John!`:

```bash
zxc greet --name John
```
