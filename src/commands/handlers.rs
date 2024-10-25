use super::command::Command;
use super::test_command::handle_test_command;

pub fn handle_command(command: Command) {
    match command {
        Command::TestCommand => handle_test_command(),
        Command::Unknown(cmd) => println!("Unknown command: {}", cmd),
    }
}

