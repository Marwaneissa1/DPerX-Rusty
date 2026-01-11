mod cheat_core;

mod game_structure;
mod ioprocesses;

use cheat_core::{CheatCore, Coords};
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct AppState {
    cheat_core: Option<CheatCore>,
    is_attached: bool,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

static GLOBAL_STATE: RwLock<Option<Arc<Mutex<AppState>>>> = RwLock::new(None);

#[derive(serde::Serialize, Clone)]
struct GameStatus {
    local_player_id: i32,
    online_players: i32,
    player_pos: Coords,
    aim_screen: Coords,
    aim_world: Coords,
}

#[tauri::command]
fn attach() -> Result<String, String> {
    let mut global = GLOBAL_STATE.write();

    if global.is_none() {
        let state = Arc::new(Mutex::new(AppState {
            cheat_core: None,
            is_attached: false,
        }));
        *global = Some(state.clone());

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));

            let mut state = state.lock();
            if state.is_attached {
                if let Some(ref mut core) = state.cheat_core {
                    core.update();
                }
            }
        });
    }

    let state_arc = global.as_ref().unwrap().clone();
    drop(global);

    let mut state = state_arc.lock();

    let core = CheatCore::new();
    state.cheat_core = Some(core);
    state.is_attached = true;
    Ok("Successfully attached to process".to_string())
}

#[tauri::command]
fn unattach() -> Result<String, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        state.is_attached = false;
        state.cheat_core = None;
        Ok("Successfully unattached from process".to_string())
    } else {
        Err("Not currently attached".to_string())
    }
}

#[tauri::command]
fn get_attach_status() -> Result<bool, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();
        Ok(state.is_attached)
    } else {
        Ok(false)
    }
}

#[tauri::command]
fn get_game_status() -> Result<GameStatus, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();

        if let Some(ref core) = state.cheat_core {
            let status = GameStatus {
                local_player_id: core.get_local_player_id(),
                online_players: core.get_online_players(),
                player_pos: core.get_player_pos(),
                aim_screen: core.get_aim_screen(),
                aim_world: core.get_aim_world(),
            };
            return Ok(status);
        } else {
            return Err("CheatCore is None".to_string());
        }
    } else {
        return Err("Global state is None".to_string());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(unused)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            attach,
            unattach,
            get_attach_status,
            get_game_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
