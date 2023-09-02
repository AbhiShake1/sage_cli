use std::env;
use std::fs::File;
use std::process::exit;
use clap::{arg, Command};

fn main() {
    let matches = Command::new("Sage CLI")
        .version("1.0")
        .author("AbhiShake1")
        .about("CLI to interact with the sage project")
        .arg(arg!(-f --feature <featurename> "Name of feature to add").required(true))
        .get_matches();

    let feature = matches.get_one::<String>("feature").expect("required");

    let project_root = match find_project_root() {
        None => {
            eprintln!("Project root not found. Are you in a flutter project?");
            exit(0)
        }
        Some(root) => root
    };

    let route_file_path = format!("{project_root}/pubspec.yaml");
    let route_file = File::open(route_file_path).expect("File not found");

    println!("feature: {:?}", feature);
    println!("project root path: {project_root}")
}

fn find_project_root() -> Option<String> {
    let current_dir = env::current_dir().ok()?;

    let mut current_path = current_dir.as_path();

    loop {
        let git_folder = current_path.join(".git");
        let pubspec_file = current_path.join("pubspec.yaml");

        if git_folder.exists() && pubspec_file.exists() {
            return Some(current_path.to_string_lossy().to_string());
        }

        if let Some(parent_path) = current_path.parent() {
            current_path = parent_path;
        } else {
            break;
        }
    }

    None
}
