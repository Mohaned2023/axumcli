pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let service_path: String = super::get_project_path() + "/src/services";
        if super::check_folder(&service_path) {
            let full_path = format!("{}/{}_service.rs", service_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", service_path);
        }
    }
}