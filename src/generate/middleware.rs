pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let middleware_path: String = super::get_project_path() + "/src/middlewares";
        if super::check_folder(&middleware_path) {
            let full_path = format!("{}/{}_middleware.rs", middleware_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", middleware_path);
        }
    }
}