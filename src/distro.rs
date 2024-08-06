use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
pub enum Distro
{
    Arch,
    Debian,
    Unknown,
}

pub fn detect_distro() -> Distro
{
    if Path::new("/etc/arch-release").exists() {
        return Distro::Arch;
    }

    if let Ok(file) = File::open("/etc/os-release") {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if line.starts_with("ID=debian") || line.starts_with("ID_LIKE=debian") {
                    return Distro::Debian;
                }
                if line.starts_with("ID=arch") || line.starts_with("ID_LIKE=arch") {
                    return Distro::Arch;
                }
            }
        }
    }

    Distro::Unknown
}
