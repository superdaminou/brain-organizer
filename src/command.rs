use std::fmt::{self};

use crate::application_error::ApplicationError;


#[derive(PartialEq, Debug)]
pub enum Command {
    Import,
    Gui,
    Export,
    Web
}

impl TryFrom<String> for Command {
    type Error = ApplicationError;
    fn try_from(value: String) -> Result<Self, ApplicationError> {
        return match value.to_lowercase().as_str() {
            "import" => Ok(Command::Import),
            "export" => Ok(Command::Export),
            "gui" => Ok(Command::Gui),
            "web" => Ok(Command::Web),
            _ => Err(ApplicationError::EnumError(value))
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Import => write!(f, "Import"),
            Command::Gui => write!(f, "GUI"),
            Command::Export => write!(f, "Export"),
            Command::Web => write!(f, "Web")
        }
    }
}

impl TryFrom<Vec<String>> for Command {
    type Error = ApplicationError;
    fn try_from(value: Vec<String>) -> Result<Self, ApplicationError> {

        value.first()
            .ok_or(ApplicationError::EnumError("".to_string()))
            .and_then(|command|Command::try_from(command.to_owned()))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_from_str() {
        assert_eq!(Command::try_from("ImPoRt".to_string()).unwrap(), Command::Import);
    }

    #[test]
    fn command_from_vec_str() {
        assert_eq!(Command::try_from(vec!["ExPoRt".to_string()]).unwrap(), Command::Export);
    }

    #[test]
    fn command_from_string() {
        assert_eq!(Command::Gui.to_string(), "GUI".to_string());
    }
}
