use std::io::{Read, Write};

pub fn get_project_path() -> String {
    std::env::current_dir()
        .expect("Error: Can NOT get the current path!")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn check_folder(path: &str) -> bool {
    match std::fs::exists(path) {
        Ok(existing) => existing,
        Err(_) => false
    }
}

pub fn create_file(path: String) -> bool {
    // if file is exists.
    match std::fs::exists(&path) {
        Ok(res) => {
            if !res { // file not exists
                match std::fs::File::create(path) { // creating the file
                    Ok(_) => true ,
                    Err(_) => false // can not create the file
                }
            } else { // file is exists
                false
            }
        },
        Err(_) => false // can not check the file
    }
}

pub fn prepend_file(data: &[u8], file_path: String ) {
    let mut f =  std::fs::File::open(&file_path)
        .expect(&format!("Error: Can NOT open the file '{}'!", file_path));
    let mut content = data.to_owned();
    f.read_to_end(&mut content)
        .expect(&format!("Error: Can NOT read the file '{}'!", file_path));

    let mut f = std::fs::File::create(&file_path)
        .expect(&format!("Error: Can NOT open the file '{}'!", file_path));
    f.write_all(content.as_slice())
        .expect(&format!("Error: Can NOT write to the file '{}'!", file_path));
}

pub fn get_name_from_args(command: &clap::ArgMatches) -> String {
    command
        .get_one::<String>("NAME")
        .expect("Error: Can NOT get the file name from the args!")
        .replace(" ", "_")
}

pub fn run_cargo_commands(args: Vec<&str>) {
    let  output: std::process::Output = std::process::Command::new("cargo")
    .args(args)
    .output()
    .expect("Error: Can NOT run 'cargo'!");

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();

    // check the command status..
    if !output.status.success() {
        eprintln!("Error: Cargo command return an error with code {}!", output.status.code().unwrap());
        std::process::exit(1);
    }
}

pub fn create_file_with_parent(path: &str) {
    if let Some( parent_dir ) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent_dir)
            .expect("Error: Can NOT create the directories!");
    }
    std::fs::File::create(path)
        .expect("Error: Can NOT create the file!");
}

pub fn get_args_from_command(command: &clap::ArgMatches) -> (Vec<String>, String) {
    let name = get_name_from_args(command);
    let models: [&str; 9] = [
        "route", "service", "state", "middleware",
        "handler", "error", "entity", "dto", "config"
    ];
    if command.get_one::<bool>("all").copied().unwrap_or(false) {
        return (
            models
                .map(|x: &str| x.to_owned())
                .to_vec(), 
            name
        );
    }
    let mut slected_models: Vec<String> = Vec::new();
    for model in models {
        if command.get_one(&model).copied().unwrap_or(false){
            slected_models.push(model.to_owned());
        }
    }
    (slected_models, name.clone())
}