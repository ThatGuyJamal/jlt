use std::process::{Command, Stdio};

use crate::distro::{self, detect_distro};
use crate::state::{CommandRunArgs, CMD_LIST};

pub fn handle_help(args: CommandRunArgs)
{
    if args.is_empty() {
        println!("Available commands:");
        for cmd in CMD_LIST {
            println!("{} - {}", cmd.name, cmd.description);
        }
    } else {
        let cmd_name = &args[0];
        match CMD_LIST.iter().find(|&&ref c| c.name == cmd_name.as_str()) {
            Some(cmd) => {
                println!("Command: {}", cmd.name);
                println!("Description: {}", cmd.description);
                println!("Example: {}", cmd.example);
                println!("Minimum arguments: {}", cmd.min_args);
                println!("Maximum arguments: {}", cmd.max_args);
            }
            None => eprintln!("Error: Command '{}' not found.", cmd_name),
        }
    }
}

/// Handles the installation of a package on the system using the appropriate package manager.
///
/// # Arguments
///
/// * `args` - A vector containing the package name as the first element.
///
/// # Panics
///
/// This function will panic if `args` is empty or if the first element of `args` is not a valid package name.
///
/// # Errors
///
/// This function will print an error message and return if the distribution is unknown,
/// if there is an error spawning the process, or if the process returns a non-success status code.
pub fn handle_install(args: CommandRunArgs)
{
    if args.is_empty() {
        eprintln!("No package name provided for installation.");
        return;
    }

    let package_name = &args[0];

    // Determine the appropriate command based on the detected distribution
    let cmd = match detect_distro() {
        distro::Distro::Arch => Command::new("bash")
            .arg("-c")
            .arg(format!("sudo pacman -S --noconfirm {}", package_name))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn(),
        distro::Distro::Debian => Command::new("bash")
            .arg("-c")
            .arg(format!("apt-get install -y {}", package_name))
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn(),
        distro::Distro::Unknown => {
            eprintln!("Unknown distribution. Cannot install package.");
            return;
        }
    };

    // Handle the result of the command execution
    match cmd {
        Ok(mut child) => match child.wait() {
            Ok(status) => {
                if status.success() {
                    println!("Successfully installed package: {}", package_name);
                } else {
                    eprintln!("Installation failed with status: {}", status);
                }
            }
            Err(e) => eprintln!("Failed to wait on child process: {}", e),
        },
        Err(e) => eprintln!("Failed to spawn process: {}", e),
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_handle_install_successful_install()
    {
        // Assuming you have some way to mock Command
        let args: CommandRunArgs = vec!["nano".to_string()];
        let output = std::panic::catch_unwind(|| handle_install(args));
        assert!(output.is_ok(), "handle_install should handle successful installation");
    }
}
