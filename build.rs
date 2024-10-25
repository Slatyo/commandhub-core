use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let commands_dir = PathBuf::from("src/commands");

    let mut mod_file = String::new();

    // List of files to exclude from automatic module generation
    let exclude_files = vec![
        "mod.rs",
        "registry.rs",
        "command.rs",
        "macros.rs",
        "commands_generated.rs",
    ];

    for entry in fs::read_dir(&commands_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if !exclude_files.contains(&filename) && filename.ends_with(".rs") {
                    let module_name = filename.trim_end_matches(".rs");

                    // Generate module code that includes the command file
                    let module_code = format!(
                        "pub mod {} {{\n    include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/commands/{}\"));\n}}\n",
                        module_name,
                        filename
                    );

                    mod_file.push_str(&module_code);
                }
            }
        }
    }

    let dest_path = out_dir.join("commands_generated.rs");
    fs::write(&dest_path, mod_file).unwrap();

    println!("cargo:rerun-if-changed=src/commands");
}
