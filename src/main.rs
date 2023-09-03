use std::{env, fs};
use std::collections::HashSet;
use std::fs::{create_dir_all, File};
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
        Some(msg) => println!("{}", msg)
    };

    match add_feature(feature, &project_root) {
        None => println!("failed to create directories"),
        Some(msg) => println!("{}", msg),
    };

    // println!("feature: {:?}", feature);
    // println!("project root path: {project_root}");
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

fn add_feature(feature_name: &str, root: &str) -> Option<String> {
    let base = format!("{root}/lib/feature/{feature_name}");
    let base_data = format!("{base}/data");
    let base_feature = format!("{base}/{feature_name}");
    let base_data_feature = format!("{base_data}/{feature_name}");
    let page = format!("{base_feature}_page.dart");
    let provider = format!("{base_feature}_provider.dart");
    let dao = format!("{base_data_feature}_dao.dart");
    let repo = format!("{base_data_feature}_repo.dart");
    let service = format!("{base_data_feature}_service.dart");

    let feature = to_title_case(feature_name);

    create_dir_all(base_data).expect("couldnt create base directory");

    File::create(&page).expect("Failed to create page");
    fs::write(&page, format!(r##"
import 'package:edm/base/base.dart';

typedef {0}PageArgs = ();

class {0}Page extends SageWidget<{0}PageArgs> {{
    @override
    Future<Widget> build(BuildContext context, WidgetRef ref) {{
        // TODO: implement build
        throw UnimplementedError();
    }}
}}

"##, feature)).expect("Failed to write to page");
    dart_format(&page);

    File::create(&provider).expect("Failed to create provider");
    fs::write(&provider, format!(r##"
import 'package:edm/base/base.dart';
import 'package:edm/feature/{0}/data/{0}_repo.dart';

final {0}Provider = Provider((ref) => {1}Provider(ref.read({0}RepoProvider)));

class {1}Provider extends SageProvider<{1}Repo> {{
  {1}Provider(super.ref);

}}
    "##, feature_name, feature)).expect("Failed to write to provider");
    dart_format(&provider);

    File::create(&dao).expect("Failed to create dao");
    fs::write(&dao, format!(r##"
import 'package:edm/base/base.dart';

final {0}DaoProvider = Provider((_) => {1}Dao());

class {1}Dao extends ModelDaoSet<{1}Model> {{}}

    "##, feature_name, feature)).expect("Failed to write to dao");
    dart_format(&dao);

    File::create(&repo).expect("Failed to create repo");
    fs::write(&repo, format!(r##"
import 'package:edm/base/base.dart';
import 'package:edm/feature/{0}/data/{0}_dao.dart';
import 'package:edm/feature/{0}/data/{0}_service.dart';

final {0}RepoProvider = Provider(
  (ref) => {1}Repo(
    service: ref.read({0}ServiceProvider),
    dao: ref.read({0}DaoProvider),
  ),
);

class {1}Repo extends SageRepo<{1}Service, {1}Dao> {{
  {1}Repo({{required super.service, required super.dao}});
}}

    "##, feature_name, feature)).expect("Failed to write to repo");
    dart_format(&repo);

    File::create(&service).expect("Failed to create service");
    fs::write(&service, format!(r##"
import 'package:edm/base/base.dart';

final {0}ServiceProvider = Provider({1}Service.new);

class {1}Service extends SageService {{
  {1}Service(super.ref);
}}

    "##, feature_name, feature)).expect("Failed to write to service");
    dart_format(&service);

    return Some("Created files".to_string());
}

// macro_rules! format {
//     ($($arg:tt)*) => {{
//         $crate::fmt::format($crate::__export::format_args!($($arg)*));
//     }}
// }
fn add_feature_to_route_file(feature_name: &str, file_path: &str) -> Option<String> {
    let import_statement = format!("import 'package:edm/feature/{0}/{0}_page.dart';", feature_name);
    let enum_statement = format!("{feature_name}<{}PageArgs>(),", to_title_case(feature_name));
    let binding = fs::read_to_string(file_path).expect("cant read");

    let imports: Vec<_> = binding
        .lines()
        .filter(|l| l.starts_with("import "))
        .collect();

    let mut new_imports = vec![import_statement];
    new_imports.extend(imports.iter().map(|s| s.to_string()));
    new_imports.sort();

    replace_in_file(
        file_path,
        imports.join("\n"), new_imports
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter().collect::<Vec<_>>()
            .join("\n"),
    );

    let enum_values = get_enum_values(file_path).expect("couldnt find enum");
    let mut new_enum_values = vec![enum_statement];
    new_enum_values.extend(enum_values.iter().cloned());
    new_enum_values.sort();

    replace_in_file(file_path, enum_values.join("\n"), new_enum_values.join("\n"));

    dart_format(file_path);

    return Some("Updated routes.dart".to_string());
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

fn dart_format(file_path: &str) {
    std::process::Command::new("dart").arg("format").arg(file_path).output().expect("failed to run dart format");
}
