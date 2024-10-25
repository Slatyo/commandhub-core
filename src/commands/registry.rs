use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::Command;
use lazy_static::lazy_static;

lazy_static! {
    static ref COMMAND_REGISTRY: Mutex<HashMap<&'static str, Arc<dyn Command + Send + Sync>>> =
        Mutex::new(HashMap::new());
}

pub fn register_command(cmd: Arc<dyn Command + Send + Sync>) {
    let mut registry = COMMAND_REGISTRY.lock().unwrap();
    registry.insert(cmd.name(), cmd);
}

pub fn get_command(name: &str) -> Option<Arc<dyn Command + Send + Sync>> {
    let registry = COMMAND_REGISTRY.lock().unwrap();
    registry.get(name).cloned()
}
