mod command;

fn main() {
    let matches = command::axum_command().get_matches();

    match matches.subcommand() {
        Some(("generate", _)) => todo!("To matches::generate"),
        _ => println!("Run with --help for usage info."),
    }
}