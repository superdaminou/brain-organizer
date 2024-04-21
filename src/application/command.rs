use std::fmt;
pub enum Command {
    IMPORT,
    GUI,
    EXPORT
}



impl From<String> for Command {
    fn from(value: std::string::String) -> Self {
        match value.to_lowercase().as_str() {
            "import" => Command::IMPORT,
            "export" => Command::EXPORT,
            "gui" => Command::GUI,
            _ => Command::GUI
        }
    }
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

