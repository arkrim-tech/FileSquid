//! FileSquid desktop entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    filesquid_desktop_lib::run();
}
