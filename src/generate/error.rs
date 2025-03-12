pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let error_path: String = super::get_project_path() + "/src/errors";
        if super::check_folder(&error_path) {
            let full_path = format!("{}/{}_error.rs", error_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", error_path);
        }
    }
}