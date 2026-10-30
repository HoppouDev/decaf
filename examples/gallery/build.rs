use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

fn main() {
    let ui_dir = PathBuf::from(
        env::var_os("DEP_DECAF_UI_DIR").expect("decaf must export the UI_DIR build metadata"),
    );

    let libraries = HashMap::from([("decaf".to_string(), ui_dir)]);

    slint_build::compile_with_config(
        "ui/app.slint",
        slint_build::CompilerConfiguration::new().with_library_paths(libraries),
    )
    .expect("Slint build failed");
}
