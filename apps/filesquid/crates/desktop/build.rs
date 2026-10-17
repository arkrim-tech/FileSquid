//! Tauri build script.
//!
//! The desktop host defines no `#[tauri::command]`s: the UI reaches the
//! FileSquid API through the `filesquid://` custom URI scheme, so there is no
//! `AppManifest::commands` list and no `permissions/` directory. The only
//! capability (`capabilities/default.json`) grants the main window event
//! listen/unlisten. If a custom command is ever added, list it in
//! `AppManifest::commands` and define its permission under `permissions/`.
//!
//! Create a stub `frontendDist` so `generate_context!` works before the first
//! Vite build of `apps/filesquid/ui`.

use std::fs;
use std::path::Path;

fn main() {
    let dist = Path::new("../../ui/dist");
    if !dist.join("index.html").exists() {
        fs::create_dir_all(dist).expect("ui dist dir");
        fs::write(
            dist.join("index.html"),
            "<!doctype html><title>FileSquid</title>\n",
        )
        .expect("ui dist stub");
    }
    tauri_build::build();
}
