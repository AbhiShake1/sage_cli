use clap::{arg, Command};

fn main() {
    let matches = Command::new("Sage CLI")
        .version("1.0")
        .author("AbhiShake1")
        .about("CLI to interact with the sage project")
        .arg(arg!(-f --feature <featurename>).required(true))
        .get_matches();

    println!(
        "feature: {:?}",
        matches.get_one::<String>("feature").expect("required")
    );
}