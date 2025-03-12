pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let config_path: String = super::get_project_path() + "/src/configs";
        if super::check_folder(&config_path) {
            let full_path = format!("{}/{}_config.rs", config_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", config_path);
        }
    }
}