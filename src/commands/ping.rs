use super::{register_command, Command};
use ctor::ctor;
use std::sync::Arc;

pub struct Ping;

impl Command for Ping {
    fn name(&self) -> &'static str {
        "PING"
    }

    fn execute(&self) {
        println!("Pong!");
    }
}

#[ctor]
fn init() {
    register_command(Arc::new(Ping));
}
