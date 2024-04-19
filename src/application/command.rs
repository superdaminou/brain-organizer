use std::fmt;

use log::info;

pub fn match_command(command: &str) -> Command{
    info!("command: {}", command);
    match command {
        "import" => Command::IMPORT,
        "export" => Command::EXPORT,
        "GUI" => Command::GUI,
        _ => panic!("Unrecognized command")
    }
}

pub enum Command {
    IMPORT,
    GUI,
    EXPORT
}



impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::IMPORT => write!(f, "Import"),
            Command::GUI => write!(f, "GUI"),
            Command::EXPORT => write!(f, "Export"),
        }
    }
}