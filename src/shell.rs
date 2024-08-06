use std::io::{self, BufRead, Write};

use crate::state::CommandRunArgs;

pub fn start()
{
    println!("Entering interactive shell mode. Type 'exit' to quit.");
    println!("Run 'help' to see a list of available commands.");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input == "clear" {
            clear_shell();
            continue;
        }

        let input_args: CommandRunArgs = input.split_whitespace().map(String::from).collect();

        if input_args.is_empty() {
            continue;
        }

        crate::run(input_args);
    }
}

pub fn clear_shell()
{
    print!("\x1B[2J\x1B[1;1H");
    println!("==== JLT Shell ====")
}
