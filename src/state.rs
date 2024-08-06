use crate::handler::{handle_help, handle_install};
use crate::shell::clear_shell;

pub type CommandRunArgs = Vec<String>;
pub type CommandRunHandler = fn(CommandRunArgs);

pub const CMD_LIST: &[CommandState] = &[
    CommandState::new(
        "help",
        "Useful information about commands and utils of the script",
        "help [command name]",
        0,
        1,
        handle_help,
    ),
    CommandState::new(
        "install",
        "Uses your system's package manager to install a program",
        "install [program name]",
        1,
        1,
        handle_install,
    ),
];

#[derive(Debug, PartialEq)]
pub struct CommandState<'a>
{
    pub name: &'a str,
    pub description: &'a str,
    pub example: &'a str,
    pub min_args: usize,
    pub max_args: usize,
    handler: CommandRunHandler,
}

impl<'a> CommandState<'a>
{
    const fn new(
        name: &'a str,
        description: &'a str,
        example: &'a str,
        min_args: usize,
        max_args: usize,
        handler: CommandRunHandler,
    ) -> CommandState<'a>
    {
        Self {
            name,
            description,
            example,
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

        clear_shell();

        (self.handler)(args);
    }
}
