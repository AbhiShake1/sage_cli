use clap::Command;

pub fn SageCommand() -> Command {
    Command::new("Sage CLI")
        .version("1.0")
        .author("AbhiShake1")
        .about("CLI to interact with the sage project")
}