pub fn match_command(string: &str) -> Command{
    match string {
        "NEXT" => Command::NEXT,
        "ADD" => Command::ADD,
        _ => panic!("Unrecognized command")
    }
}



pub enum Command {
    NEXT,
    ADD,
    DB
}

