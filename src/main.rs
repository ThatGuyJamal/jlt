use std::env;

use commands::{CommandRunArgs, CMD_LIST};

mod commands;
mod distro;
mod handler;
mod shell;
mod utils;

fn main()
{
    let args: CommandRunArgs = env::args().collect();

    if args.contains(&"--shell".to_string()) {
        shell::start();
    } else {
        // Skip the first argument (the program name) for single-command mode
        if args.len() < 2 {
            eprintln!("Error: No command provided. Please specify a command.");
            return;
        }

        let cmd_args: CommandRunArgs = args[1..].to_vec();

        run(cmd_args);
    }
}

pub fn run(args: CommandRunArgs)
{
    if args.is_empty() {
        eprintln!("Error: No command provided. Please specify a command.");
        return;
    }

    let cmd = &args[0];
    let cmd_args = args[1..].to_vec();

    match CMD_LIST.iter().find(|&&ref c| c.name == cmd.as_str() && c.enabled) {
        Some(command) => command.prepare(cmd_args),
        None => eprintln!("Error: Command '{}' not found.", cmd),
    }
}
