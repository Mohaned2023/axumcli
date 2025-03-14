mod command;
mod matches;
mod generate;
mod utilities;
mod new;

fn main() {
    let matches = command::axum_command().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_command)) => matches::generate_matches(sub_command),
        Some(("new", command)) => new::new_axum_project(command.get_one::<String>("PATH").unwrap()),
        _ => println!("Run with --help for usage info."),
    }
}