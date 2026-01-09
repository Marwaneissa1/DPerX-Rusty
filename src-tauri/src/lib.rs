mod cheat_core;

mod game_structure;
mod ioprocesses;

#[tauri::command]
fn attach() {
    cheat_core::attach();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(unused)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![attach])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
