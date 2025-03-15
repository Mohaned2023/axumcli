use clap::ArgMatches;
use super::generate;

pub fn generate_matches(sub_command: &ArgMatches) {
    match sub_command.subcommand() {
        Some(("model", command)) => generate::new_model( super::utilities::get_args_from_command(command) ),
        Some((g_type, command)) => generate::new(g_type,&super::utilities::get_name_from_args(command)),
        _ => eprintln!("Run with 'axumcli generate help' for usage information.")
    }
}