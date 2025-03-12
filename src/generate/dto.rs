pub fn generate(option_name: Option<&String>) {
    if let Some(name) = option_name {
        let dto_path: String = super::get_project_path() + "/src/dtos";
        if super::check_folder(&dto_path) {
            let full_path = format!("{}/{}_dto.rs", dto_path, name);
            todo!("Created at '{}'", full_path);
        } else {
            println!("Note: Make sure you are in the root folder of the project!");
            eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", dto_path);
        }
    }
}