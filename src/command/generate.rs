use super::{Command, arg};

pub fn generate_commands() -> Vec<Command> {
    vec! [
        Command::new("route")
                    .alias("r")
                    .about("New Route.")
                    .arg(arg!(<NAME> "The Route Name.")),
        Command::new("service")
                    .alias("s")
                    .about("New Service.")
                    .arg(arg!(<NAME> "The Service Name.")),
        Command::new("state")
                    .alias("st")
                    .about("New State.")
                    .arg(arg!(<NAME> "The State Name.")),
        Command::new("middleware")
                    .alias("m")
                    .about("New Middleware.")
                    .arg(arg!(<NAME> "The Middleware Name.")),
        Command::new("handler")
                    .alias("hd")
                    .about("New Handler.")
                    .arg(arg!(<NAME> "The Handler Name.")),
        Command::new("error")
                    .alias("er")
                    .about("New Error.")
                    .arg(arg!(<NAME> "The Error Name.")),
        Command::new("entity")
                    .alias("en")
                    .about("New Entity.")
                    .arg(arg!(<NAME> "The Entity Name.")),
        Command::new("dto")
                    .about("New DTO.")
                    .arg(arg!(<NAME> "The DTO Name.")),
        Command::new("config")
                    .alias("cf")
                    .about("New Config.")
                    .arg(arg!(<NAME> "The Config Name.")),
    ]
}