use crate::state::{CommandRunArgs, CMD_LIST};

pub fn handle_help(_args: CommandRunArgs)
{
    println!(
        "Commands: {}",
        CMD_LIST.iter().map(|c| c.name).collect::<Vec<&str>>().join(", ")
    );
}

pub fn handle_install(args: CommandRunArgs)
{
    println!("Installing {}...", args[0]);
}