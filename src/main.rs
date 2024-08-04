#[derive(Debug, PartialEq)]
struct Command<'a> {
    name: &'a str,
    description: &'a str,
}

impl<'a> Command<'a> {
    const fn new(name: &'a str, description: &'a str) -> Command<'a> {
        Command {
            name,
            description,
        }
    }

    fn run(&self, args: Option<Vec<String>>) {
        println!("Running command: '{}' with args: {:?}", self.name, args);
    }
}

const CMD_LIST: [Command; 2] = [
    Command::new("help", "Display help"),
    Command::new("install", "Install dependencies"),
];

fn main() {
    let cmd = std::env::args().nth(1).unwrap(); // The command name we want to call
    let args: Vec<String> = std::env::args().skip(2).collect(); // The arguments passed to the command

    if let Some(command) = CMD_LIST.iter().find(|&c| c.name == cmd) {
        if args.is_empty() {
            command.run(None);
        } else {
            command.run(Some(args));
        }
    } else {
        eprintln!("Command not found: {}", cmd);
    }
}
