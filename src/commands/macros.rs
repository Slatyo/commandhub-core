#[macro_export]
macro_rules! register_command {
    ($command_struct:ident) => {{
        use $crate::commands::registry::register_command;
        register_command(Box::new($command_struct));
    }};
}
