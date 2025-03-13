mod command;
mod matches;
mod generate;
mod utilities;

fn main() {
    let matches = command::axum_command().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_command)) => matches::generate_matches(sub_command),
        _ => println!("Run with --help for usage info."),
    }
}