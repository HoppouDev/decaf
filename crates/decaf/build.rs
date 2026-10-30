use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"));

    // The ui directory ships the .slint sources consumers compile against, e.g.
    // `import { Button } from "@decaf/components/button.slint";`
    println!("cargo::rerun-if-changed=ui");
    println!(
        "cargo::metadata=UI_DIR={}",
        manifest_dir.join("ui").display()
    );
}
