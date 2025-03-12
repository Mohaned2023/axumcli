use clap::ArgMatches;
use super::generate;

pub fn generate_matches(sub_command: &ArgMatches) {
    match sub_command.subcommand() {
        Some(("route",      command)) => generate::route::generate(command.get_one::<String>("NAME")),
        Some(("service",    command)) => generate::service::generate(command.get_one::<String>("NAME")),
        Some(("state",      command)) => generate::state::generate(command.get_one::<String>("NAME")),
        Some(("middleware", command)) => generate::middleware::generate(command.get_one::<String>("NAME")),
        Some(("handler",    command)) => generate::handler::generate(command.get_one::<String>("NAME")),
        Some(("error",      command)) => generate::error::generate(command.get_one::<String>("NAME")),
        Some(("entity",     command)) => generate::entity::generate(command.get_one::<String>("NAME")),
        Some(("dto",        command)) => generate::dto::generate(command.get_one::<String>("NAME")),
        Some(("config",     command)) => generate::config::generate(command.get_one::<String>("NAME")),
        _ => eprintln!("Run with 'axumcli generate help' for usage info.")
    }
}