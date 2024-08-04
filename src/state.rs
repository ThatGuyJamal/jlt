use crate::commands::{handle_help, handle_install};

pub type CommandRunArgs = Vec<String>;
pub type CommandRunHandler = fn(CommandRunArgs);

pub const CMD_LIST: &[CommandState] = &[
    CommandState::new("help", "Display help", 0, 0, handle_help),
    CommandState::new("install", "Install a program", 1, 1, handle_install),
];

#[derive(Debug, PartialEq)]
pub struct CommandState<'a>
{
    pub name: &'a str,
    description: &'a str,
    min_args: usize,
    max_args: usize,
    handler: CommandRunHandler,
}

impl<'a> CommandState<'a>
{
    const fn new(
        name: &'a str,
        description: &'a str,
        min_args: usize,
        max_args: usize,
        handler: CommandRunHandler,
    ) -> CommandState<'a>
    {
        Self {
            name,
            description,
            min_args,
            max_args,
            handler,
        }
    }

    pub fn prepare(&self, args: CommandRunArgs)
    {
        if args.len() < self.min_args {
            eprintln!(
                "Error: Command '{}' requires at least {} arguments but {} were provided.",
                self.name,
                self.min_args,
                args.len()
            );
            return;
        }

        if args.len() > self.max_args {
            eprintln!(
                "Error: Command '{}' requires at most {} arguments but {} were provided.",
                self.name,
                self.max_args,
                args.len()
            );
            return;
        }

        (self.handler)(args);
    }
}
