include!(concat!(env!("OUT_DIR"), "/commands_generated.rs"));

pub mod command;
pub mod macros;
pub mod registry;

pub use registry::{get_command, register_command};

pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self);
}
