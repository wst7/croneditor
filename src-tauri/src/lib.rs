
mod core;
use core::cmd;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd::load_crons,
            cmd::save_crons,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
