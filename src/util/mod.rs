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