pub fn match_command(command: &str) -> Command{
    println!("command: {}", command);
    match command {
        "import" => Command::IMPORT,
        "gui" => Command::GUI,
        _ => panic!("Unrecognized command")
    }
}



pub enum Command {
    IMPORT,
    GUI
}

