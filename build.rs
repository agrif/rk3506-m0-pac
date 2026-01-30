use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // If rt feature is enabled, use device.x
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        // Put `device.x` in our output directory and ensure it's
        // on the linker search path.
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("device.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());

        // By default, Cargo will re-run a build script whenever
        // any file in the project changes. By specifying `device.x`
        // here, we ensure the build script is only re-run when
        // `device.x` is changed.
        println!("cargo:rerun-if-changed=device.x");
    }

    // Also rebuild if `build.rs` is changed.
    println!("cargo:rerun-if-changed=build.rs");
}
