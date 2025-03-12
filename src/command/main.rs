use super::{Command, generate};

pub fn main_commands() -> Vec<Command> {
    vec![
        Command::new("generate")
                    .alias("g")
                    .about("generate about")
                    .subcommands(generate::generate_commands()),
    ]
}