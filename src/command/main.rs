use clap::arg;

use super::{Command, generate};

pub fn main_commands() -> Vec<Command> {
    vec![
        Command::new("generate")
                    .alias("g")
                    .about("Generate new template (e.g. route, middleware, handler ...)")
                    .subcommands(generate::generate_commands()),
        Command::new("new")
                .alias("n")
                .about("Create/Initialize new axum project.")
                .arg(arg!(<PATH> "The project path."))
    ]
}