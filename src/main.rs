use clap::arg;

use commands::{Commands, SageCommand};

mod commands;
mod utils;

fn main() {
    let matches = SageCommand::new()
        .arg(arg!(-f --feature <featurename> "Name of feature to add").required(true))
        .get_matches();

    let feature = matches.get_one::<String>("feature");

    // TODO(AbhiShake1): refactor
    if let Some(feat) = feature {
        Commands::Feature(Box::from(feat.to_string()));
    }
}
