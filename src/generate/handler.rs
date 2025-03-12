pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let handler_path: String = super::get_project_path() + "/src/handlers";
        if super::check_folder(&handler_path) {
            let full_path = format!("{}/{}_handler.rs", handler_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", handler_path);
        }
    }
}