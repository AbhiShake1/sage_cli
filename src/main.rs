use clap::{arg, Command};

use commands::Commands;

mod commands;
mod utils;

fn main() {
    let matches = Command::new("Sage CLI")
        .version("1.0")
        .author("AbhiShake1")
        .about("CLI to interact with the sage project")
        .arg(arg!(-f --feature <featurename> "Name of feature to add").required(true))
        .get_matches();

    let feature = matches.get_one::<String>("feature");

    // TODO(AbhiShake1): refactor
    if let Some(feat) = feature {
        Commands::Feature(Box::from(feat.to_string()));
    }
}
