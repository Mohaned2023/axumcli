mod generate;
mod main;
use clap::{Command, arg};

pub fn axum_command() -> Command {
    Command::new("axumcli")
        .about("This is the axumcli.")
        .version("0.1.0")
        .author("Mohaned Sherhan")
        .subcommands(main::main_commands() )
}