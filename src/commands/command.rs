use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    TestCommand,    
    Unknown(String),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "TEST-COMMAND" => Ok(Command::TestCommand),
            _ => Ok(Command::Unknown(input.to_string())),
        }
    }
}
