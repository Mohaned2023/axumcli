pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let entity_path: String = super::get_project_path() + "/src/entities";
        if super::check_folder(&entity_path) {
            let full_path = format!("{}/{}_entity.rs", entity_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", entity_path);
        }
    }
}