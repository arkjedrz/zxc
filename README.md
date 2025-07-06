# `zxc` - directory-oriented command runner

`zxc` is a directory-oriented command runner.
This means it is able to run commands defined by YAML files.

## Features

- YAML-based definition - human-readable and easy to write
- Jinja-based template substitution - flexible
- native look and feel

## Example

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

Refer to [definition file documentation](./docs/definition_file.md) for more information.

Use following command to print `Hello world!`:

```bash
zxc greet --name world
```

![demo](./docs/demo.svg)

## Installation

### From source

```bash
cargo build --release
cargo install --path .
```

### From packages

`deb`-based systems:

```bash
sudo dpkg -i zxc_<version>-1_amd64.deb
```

`rpm`-based systems:

```bash
sudo rpm -i  zxc-<version>-1.x86_64.rpm
```

## Development

```bash
pip install pre-commit
cargo install cargo-deb cargo-generate-rpm cargo-aur
```

### Pre-commit setup

Install and run hooks:

```bash
pre-commit install
pre-commit run -a
```

### Build, test and install

```bash
cargo build --release
cargo test
cargo install --path .
```

### Create packages

Application must be built and stripped:

```bash
cargo build --release
strip -s target/release/zxc
```

Create packages:

```bash
cargo deb
cargo generate-rpm
cargo aur
```
