struct Command {
    name: String,
    args: Vec<String>,
    description: String
}

impl Command {
    fn new(name: String, args: Vec<String>, description: String) -> Command {
        Command {
            name,
            args,
            description
        }
    }

    fn run(&self) {
        println!("Running command: {} with args: {:?}", self.name, self.args);
    }
}

const CMD_LIST: &[&str] = &["help", "install"];

fn main() 
{
    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        let cmd = CMD_LIST.iter().find(|&&c| arg == c);
        if let Some(c) = cmd {
            println!("Found command: {}", c);
        } else {
            println!("Unknown command: {}", arg);
        }
    }

    println!("")
}
