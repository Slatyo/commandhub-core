use super::{register_command, Command};
use ctor::ctor;
use std::sync::Arc;

pub struct TestCommand;

impl Command for TestCommand {
    fn name(&self) -> &'static str {
        "TEST-COMMAND"
    }

    fn execute(&self) {
        println!("TestCommand executed!");
    }
}

#[ctor]
fn init() {
    register_command(Arc::new(TestCommand));
}
