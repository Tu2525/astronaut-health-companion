// Pure shell: no commands, no logic. The web app in apps/web/dist does everything.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running ZeroG Pulse");
}
