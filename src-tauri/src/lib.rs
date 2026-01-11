mod cheat_core;
mod overlay;

mod game_structure;
mod ioprocesses;

use cheat_core::{CheatCore, Coords};
use overlay::{DrawCircle, DrawLine, OverlayState};
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
static OVERLAY_STATE: RwLock<Option<Arc<Mutex<OverlayState>>>> = RwLock::new(None);

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
                    if let Err(e) = core.update() {
                        eprintln!("Memory read failed, auto-detaching: {}", e);
                        state.is_attached = false;
                        state.cheat_core = None;
                    }
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

#[tauri::command]
fn start_overlay() -> Result<String, String> {
    let mut global = OVERLAY_STATE.write();

    if global.is_none() {
        let state = Arc::new(Mutex::new(OverlayState::default()));
        *global = Some(state);
        Ok("Overlay initialized".to_string())
    } else {
        Ok("Overlay already initialized".to_string())
    }
}

#[tauri::command]
fn get_overlay_state() -> Result<OverlayState, String> {
    let global = OVERLAY_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();
        Ok(state.clone())
    } else {
        Err("Overlay not initialized".to_string())
    }
}

#[tauri::command]
fn get_window_rect() -> Result<ioprocesses::WindowRect, String> {
    ioprocesses::get_window_rect_by_process_name("ddnet.exe")
}

#[tauri::command]
fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, color: [u8; 4], thickness: f32) -> Result<String, String> {
    let global = OVERLAY_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        state.lines.push(DrawLine {
            x1,
            y1,
            x2,
            y2,
            color,
            thickness,
        });
        Ok("Line added".to_string())
    } else {
        Err("Overlay not started".to_string())
    }
}

#[tauri::command]
fn draw_circle(x: f32, y: f32, radius: f32, color: [u8; 4], filled: bool, thickness: f32) -> Result<String, String> {
    let global = OVERLAY_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        state.circles.push(DrawCircle {
            x,
            y,
            radius,
            color,
            filled,
            thickness,
        });
        Ok("Circle added".to_string())
    } else {
        Err("Overlay not started".to_string())
    }
}

#[tauri::command]
fn clear_overlay() -> Result<String, String> {
    let global = OVERLAY_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        state.lines.clear();
        state.circles.clear();
        Ok("Overlay cleared".to_string())
    } else {
        Err("Overlay not started".to_string())
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
            get_game_status,
            start_overlay,
            get_overlay_state,
            get_window_rect,
            draw_line,
            draw_circle,
            clear_overlay
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
