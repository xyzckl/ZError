#[cfg(feature = "tauri-build")]
fn main() {
    tauri_build::build()
}

#[cfg(not(feature = "tauri-build"))]
fn main() {}
