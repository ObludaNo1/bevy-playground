use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);

    generate_module_bindings(manifest_dir);

    let src_assets = manifest_dir.join("src").join("assets");
    let dst_assets = manifest_dir.join("target").join(&profile).join("assets");

    if src_assets.exists() {
        if dst_assets.exists() {
            let _ = fs::remove_dir_all(&dst_assets);
        }
        let _ = copy_dir_all(&src_assets, &dst_assets);
    }

    println!("cargo:rerun-if-changed=src/assets");
    println!("cargo:rerun-if-changed=server/src");
    println!("cargo:rerun-if-changed=server/Cargo.toml");
}

fn generate_module_bindings(manifest_dir: &Path) {
    let server_dir = manifest_dir.join("server");

    ensure_spacetime_cli_available();

    let status = Command::new("spacetime")
        .current_dir(&server_dir)
        // Remove RUSTFLAGS because they contain linker flags specific to this project's targets.
        // If we pass them to spacetime, it tries to apply game-specific flags when compiling
        // the wasm module, causing linker errors (e.g., "-fuse-ld=lld" is invalid for wasm).
        .env_remove("RUSTFLAGS")
        // Remove CARGO_ENCODED_RUSTFLAGS, which is the base64-encoded version of RUSTFLAGS.
        // Cargo uses this internally, and we must clear it too to prevent the same conflict.
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .args([
            "generate",
            "--lang",
            "rust",
            "--out-dir",
            "../src/module_bindings",
            "--module-path",
            ".",
        ])
        .stdin(Stdio::null())
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            panic!(
                "spacetime generate failed with status {status}. Ensure the local Spacetime \
                 module is valid and the spacetime CLI is installed."
            );
        }
        Err(err) => {
            panic!(
                "failed to run spacetime generate: {err}. Install Spacetime CLI and ensure it is \
                 on PATH."
            );
        }
    }
}

fn ensure_spacetime_cli_available() {
    let status = Command::new("spacetime")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            panic!(
                "Spacetime CLI was found, but failed to run (status: {status}). Please verify \
                 your installation, then retry."
            );
        }
        Err(err) => {
            panic!(
                "Spacetime CLI is required to generate Rust module bindings but was not found. \
                 Install it and ensure 'spacetime' is on PATH. \
                 See installation instructions: https://spacetimedb.com/install\n\nOriginal error: {err}"
            );
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), &dst_path)?;
        }
    }
    Ok(())
}
