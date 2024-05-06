use std::fmt;
pub enum Command {
    Import,
    Gui,
    Export
}



impl From<String> for Command {
    fn from(value: std::string::String) -> Self {
        match value.to_lowercase().as_str() {
            "import" => Command::Import,
            "export" => Command::Export,
            "gui" => Command::Gui,
            _ => Command::Gui
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Import => write!(f, "Import"),
            Command::Gui => write!(f, "GUI"),
            Command::Export => write!(f, "Export"),
        }
    }
}

