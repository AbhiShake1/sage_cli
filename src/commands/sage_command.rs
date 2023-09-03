use clap::Command;

pub struct SageCommand {}

impl SageCommand {
    pub fn new() -> Command {
        Command::new("Sage CLI")
            .version("1.0")
            .author("AbhiShake1")
            .about("CLI to interact with the sage project")
    }
}
