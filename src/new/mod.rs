pub fn new_axum_project(path: &String) {
    // cargo init <path>
    crate::utilities::run_cargo_commands(vec!["init", path]);

    println!("-> Changing the path to '{}'", path);
    std::env::set_current_dir(path)
        .expect(&format!("Error: Can NOT change the path to '{}'!", path));

    // cargo add axum
    println!("-> Installing axum in '{}'", path);
    crate::utilities::run_cargo_commands(vec!["add", "axum"]);

    // cargo add tokio -F full
    println!("-> Installing tokio with full features in '{}'", path);
    crate::utilities::run_cargo_commands(vec!["add", "tokio", "-F", "full"]);

    crate::generate::init();
}