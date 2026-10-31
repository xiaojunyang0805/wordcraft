//! WordCraft desktop app: [`wordcraft::run`] with WordCraft's own [`wordcraft::AppConfig`].
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    wordcraft::run(wordcraft::AppConfig::default())
}
