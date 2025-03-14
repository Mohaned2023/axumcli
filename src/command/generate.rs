use super::{Command, arg};

pub fn generate_commands() -> Vec<Command> {
    vec! [
        Command::new("route")
                    .alias("r")
                    .about("Create new route file in routes dir and update routes/mod.rs.")
                    .arg(arg!(<NAME> "The Route Name.")),
        Command::new("service")
                    .alias("s")
                    .about("Create new service file in services dir and update services/mod.rs.")
                    .arg(arg!(<NAME> "The Service Name.")),
        Command::new("state")
                    .alias("st")
                    .about("Create new state file in states dir and update states/mod.rs.")
                    .arg(arg!(<NAME> "The State Name.")),
        Command::new("middleware")
                    .alias("m")
                    .about("Create new middleware file in middlewares dir and update middlewares/mod.rs.")
                    .arg(arg!(<NAME> "The Middleware Name.")),
        Command::new("handler")
                    .alias("hd")
                    .about("Create new handler file in handlers dir and update handlers/mod.rs.")
                    .arg(arg!(<NAME> "The Handler Name.")),
        Command::new("error")
                    .alias("er")
                    .about("Create new error file in errors dir and update errors/mod.rs.")
                    .arg(arg!(<NAME> "The Error Name.")),
        Command::new("entity")
                    .alias("en")
                    .about("Create new entity file in entitys dir and update entitys/mod.rs.")
                    .arg(arg!(<NAME> "The Entity Name.")),
        Command::new("dto")
                    .about("Create new dto file in dtos dir and update dtos/mod.rs.")
                    .arg(arg!(<NAME> "The DTO Name.")),
        Command::new("config")
                    .alias("cf")
                    .about("Create new config file in configs dir and update configs/mod.rs.")
                    .arg(arg!(<NAME> "The Config Name.")),
    ]
}