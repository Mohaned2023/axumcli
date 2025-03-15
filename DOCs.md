## Installation:
```bash
# if you install by the repo:
$ cargo install --path .

# if you install from https://crates.io/
$ cargo install axumcli
```

## Command:
- `new` Create New Axum Application.
- `generate` or `g` for generating a specific model:
    - `model`
        - `--all`
        - `--route`
        - `--service`
        - `--state`
        - `--middleware`
        - `--handler`
        - `--error`
        - `--entity`
        - `--dto`
        - `--config`
    - `route`
    - `service`
    - `state`
    - `middleware`
    - `handler`
    - `error`
    - `entity`
    - `dto`
    - `config`

## Usage:
You can use `--help` for any command to print the help message.

- Create/Initialize new axum project:
```bash
$ axumcli new <path>
# Example:
$ axumcli new ./test/first-zxum
```
- Generate new model (e.g. route, middleware, handler ...)
```bash
$ axumcli generate <type> <name>
# OR for short:
$ axumcli g <type> <name>
# Example: 
$ axumcli g route first
```
NOTE: You can see the supported models by using `$ axumcli g --help`.

NOTE: If you use the space in the name (e.g. `$ axumcli g route "first test"`) It will be
replaced wiht underscore and will add to it the postfix `_route.rs` (e.g. `first test` = `first_test_route.rs`).

- Generate Model:
Model has many flags (e.g. --all, --route, --service ...)
```bash
# generate model with all dependencies
# (e.g. route, middleware, handler ...)
$ axumcli g model --all <name>
# OR you can select one or more dependencies
$ axumcli g model --route --middleware --entity <name>
# Example:
$ axumcli g model --route --middleware --entity first
```
NOTE: You can see the supported models by using `$ axumcli g model --help`.