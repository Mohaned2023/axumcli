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