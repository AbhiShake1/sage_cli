use std::{env, fs};
use std::process::exit;

use clap::{arg, Command};

use regex::Regex;

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

    let route_file_path = format!("{project_root}/lib/base/routing/routes.dart");
    // let route_code = fs::read_to_string(route_file_path).expect("TODO: panic message");

    match add_feature_to_route_file(feature, &route_file_path) {
        None => println!("Failed to add to route file"),
        Some(_) => println!("Succeeded adding to route file")
    };

    println!("feature: {:?}", feature);
    println!("project root path: {project_root}");
    // println!("route code: {:?}", route_code);
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

fn add_feature_to_route_file(feature_name: &str, file_path: &str) -> Option<String> {
    let import_statement = format!("import 'package:edm/feature/{feature_name}/{feature_name}.dart';");
    let enum_statement = format!("{feature_name}<{}PageArgs>(),", to_title_case(feature_name));
    let binding = fs::read_to_string(file_path).expect("cant read");

    let imports: Vec<_> = binding
        .lines()
        .filter(|l| l.starts_with("import "))
        .collect();

    let previous: Vec<_> = imports.iter().filter(|i| i.contains(&import_statement)).collect();

    if !previous.is_empty() {
        return None;
    }

    let mut new_imports = vec![import_statement];
    new_imports.extend(imports.iter().map(|s| s.to_string()));
    new_imports.sort();

    replace_in_file(file_path, imports.join("\n"), new_imports.join("\n"));

    let enum_values = get_enum_values(file_path).expect("couldnt find enum");
    let mut new_enum_values = vec![enum_statement];
    new_enum_values.extend(enum_values.iter().cloned());
    new_enum_values.sort();

    println!("{:?}", new_enum_values);

    replace_in_file(file_path, enum_values.join("\n"), new_enum_values.join("\n"));

    println!("{:?}", new_enum_values);
    println!("{:?}", new_imports);

    std::process::Command::new("dart").arg("format").arg(file_path).output().expect("failed to run dart format");
    None
}

fn get_enum_values(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let re = Regex::new(r"enum\s*\w*\s*<[^>]*>\s*\{([^}]*)\}").unwrap();

    let file_content = fs::read_to_string(file_path)?;
    let enum_values: Vec<String> = re
        .captures_iter(&file_content)
        .flat_map(|captures| {
            captures.get(1).map(|match_| match_.as_str())
        })
        .map(|enum_value| {
            enum_value
                .trim()
                .trim_end_matches(",")
                .trim_end_matches(";")
                .split("\n")
                .collect::<String>()
                .trim()
                .to_owned()
        })
        .filter(|enum_value| !enum_value.is_empty())
        .collect();

    Ok(enum_values)
}

fn replace_in_file(file_path: &str, previous_lines: String, new_lines: String) {
    let bindings = fs::read_to_string(file_path).expect("couldnt read file");

    let new = bindings.replace(&previous_lines, &*new_lines);

    fs::write(file_path, new).expect("couldnt write to file");
}

fn to_title_case(input: &str) -> String {
    let mut titlecase = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphabetic() {
            if capitalize_next {
                titlecase.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                titlecase.push(c.to_ascii_lowercase());
            }
        } else {
            titlecase.push(c);
            capitalize_next = true;
        }
    }

    titlecase
}
