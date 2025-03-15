use super::{Command, arg, Arg};

pub fn generate_commands() -> Vec<Command> {
    vec! [
        Command::new("model")
                    .alias("m")
                    .about("Create new model file in models dir and update models/mod.rs.")
                    .args([
                        arg!(   <NAME>       "The model name."),
                        Arg::new("all").short('a').long("all").action(clap::ArgAction::SetTrue)
                                .help("Create all templates like: route, service, middleware, state ..."),
                        Arg::new("route").short('r').long("route").action(clap::ArgAction::SetTrue),
                        Arg::new("service").short('s').long("service").action(clap::ArgAction::SetTrue),
                        Arg::new("state").long("state").action(clap::ArgAction::SetTrue),
                        Arg::new("middleware").short('m').long("middleware").action(clap::ArgAction::SetTrue),
                        Arg::new("handler").short('n').long("handler").action(clap::ArgAction::SetTrue),
                        Arg::new("error").long("error").action(clap::ArgAction::SetTrue),
                        Arg::new("entity").short('e').long("entity").action(clap::ArgAction::SetTrue),
                        Arg::new("dto").long("dto").action(clap::ArgAction::SetTrue),
                        Arg::new("config").long("config").action(clap::ArgAction::SetTrue),
                    ]),
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
                    .alias("mw")
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