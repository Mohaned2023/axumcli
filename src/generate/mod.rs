pub fn new( g_type: &str, name: String) {
        let g_path: String = format!("{}/src/{}s", super::utilities::get_project_path(), g_type);
    if super::utilities::check_folder(&g_path) {
        let full_path = format!("{}/{}_{}.rs", g_path, name, g_type);
        if super::utilities::create_file(full_path.clone()) {
            let mod_path: String = format!("{}/mod.rs", g_path);
            println!("-> Create the {} at '{}'", g_type, full_path);
            super::utilities::create_file(mod_path.clone());
            let line = format!("pub mod {}_{};\n", name, g_type);
            println!("-> Update the {}s mod at '{}'", g_type, mod_path);
            super::utilities::prepend_file(line.as_bytes(), mod_path);
        } else {
            eprintln!("Error: file exists or can NOT be created!");
        }
    } else {
        println!("Note: Make sure you are in the root folder of the project!");
        eprintln!("Error: Can NOT create the file, No such file or directory '{}'!", g_path);
    }
}