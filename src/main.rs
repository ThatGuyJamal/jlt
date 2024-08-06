use std::env;

use state::CMD_LIST;

mod commands;
mod state;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No command provided. Please specify a command.");
        return;
    }

    let cmd: &String = &args[1];
    let cmd_args = args[2..].to_vec();

    match CMD_LIST.iter().find(|&&ref c| c.name == cmd.as_str()) {
        Some(command) => command.prepare(cmd_args),
        None => eprintln!("Error: Command '{}' not found.", cmd),
    }
}
