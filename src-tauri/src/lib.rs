mod aimbot;
mod cheat_core;
mod input_hook;

mod ioprocesses;
mod offsets;

use aimbot::{Aimbot, AimbotStatus};
use cheat_core::{CheatCore, Coords};
use input_hook::{vk_code_from_string, InputHook};
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct AppState {
    cheat_core: Option<CheatCore>,
    aimbot: Aimbot,
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
    players: Vec<PlayerInfo>,
}

#[derive(serde::Serialize, Clone)]
struct PlayerInfo {
    id: i32,
    gametick: i64,
    pos: Coords,
    vel: Coords,
}

#[tauri::command]
fn attach() -> Result<String, String> {
    let mut global = GLOBAL_STATE.write();

    if global.is_none() {
        let state = Arc::new(Mutex::new(AppState {
            cheat_core: None,
            aimbot: Aimbot::new(),
            is_attached: false,
        }));
        *global = Some(state.clone());

        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1));

            let mut state = state.lock();
            if state.is_attached {
                let (local_id, players, local_pos, local_vel, mouse_pos, should_update) = {
                    if let Some(ref mut core) = state.cheat_core {
                        if let Err(e) = core.update() {
                            eprintln!("Memory read failed, auto-detaching: {}", e);
                            state.is_attached = false;
                            state.cheat_core = None;
                            (
                                0,
                                Vec::new(),
                                Coords::default(),
                                Coords::default(),
                                Coords::default(),
                                false,
                            )
                        } else {
                            let local_id = core.get_local_player_id();
                            let players = core.get_players().to_vec();
                            let local_pos = core.get_player_pos();
                            let local_vel = core.get_local_velocity();
                            let mouse_pos = core.get_aim_screen();
                            let should_update = state.aimbot.get_config().lock().enabled;
                            (local_id, players, local_pos, local_vel, mouse_pos, should_update)
                        }
                    } else {
                        (
                            0,
                            Vec::new(),
                            Coords::default(),
                            Coords::default(),
                            Coords::default(),
                            false,
                        )
                    }
                };

                if should_update {
                    let config = state.aimbot.get_config().lock().clone();

                    let should_aim = config.always_active || InputHook::is_trigger_key_pressed();

                    if should_aim {
                        if let Some(aim_pos) = state.aimbot.update(local_id, &players, local_pos, local_vel, mouse_pos)
                        {
                            if let Some(ref core) = state.cheat_core {
                                let _ = core.write_aim_position(aim_pos);
                            }
                        }
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

#[derive(serde::Serialize)]
struct GameProcessStatus {
    process_found: bool,
    window_found: bool,
    process_name: String,
}

#[tauri::command]
fn get_game_process_status() -> Result<GameProcessStatus, String> {
    use ioprocesses::Process;

    let process_found = Process::from_process_name("ddnet.exe").is_ok();
    let window_found = if process_found {
        Process::from_window_title("DDNet").is_ok()
    } else {
        false
    };

    Ok(GameProcessStatus {
        process_found,
        window_found,
        process_name: "DDNet".to_string(),
    })
}

#[tauri::command]
fn get_game_status() -> Result<GameStatus, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();

        if let Some(ref core) = state.cheat_core {
            let players_data = core.get_players();
            let players_info: Vec<PlayerInfo> = players_data
                .iter()
                .filter(|p| p.gametick > 0)
                .map(|p| PlayerInfo {
                    id: p.id,
                    gametick: p.gametick,
                    pos: p.pos,
                    vel: p.vel,
                })
                .collect();

            let status = GameStatus {
                local_player_id: core.get_local_player_id(),
                online_players: core.get_online_players(),
                player_pos: core.get_player_pos(),
                aim_screen: core.get_aim_screen(),
                aim_world: core.get_aim_world(),
                players: players_info,
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
fn set_aimbot_enabled(enabled: bool) -> Result<String, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        let mut config = state.aimbot.get_config().lock().clone();
        config.enabled = enabled;
        state.aimbot.set_config(config);
        Ok(format!("Aimbot {}", if enabled { "enabled" } else { "disabled" }))
    } else {
        Err("Not currently attached".to_string())
    }
}

#[tauri::command]
fn set_aimbot_config(
    fov: Option<f32>,
    silent: Option<bool>,
    hook_visible: Option<bool>,
    edge_scan: Option<bool>,
    max_distance: Option<f32>,
    always_active: Option<bool>,
    prediction_enabled: Option<bool>,
    prediction_time: Option<f32>,
    target_priority: Option<String>,
    ignore_frozen: Option<bool>,
) -> Result<String, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();
        let config_arc = state.aimbot.get_config();
        let mut config = config_arc.lock();

        if let Some(fov) = fov {
            config.fov = fov;
        }
        if let Some(silent) = silent {
            config.silent = silent;
        }
        if let Some(hook_visible) = hook_visible {
            config.hook_visible = hook_visible;
        }
        if let Some(edge_scan) = edge_scan {
            config.edge_scan = edge_scan;
        }
        if let Some(max_distance) = max_distance {
            config.max_distance = max_distance;
        }
        if let Some(always_active) = always_active {
            config.always_active = always_active;
        }
        if let Some(prediction_enabled) = prediction_enabled {
            config.prediction_enabled = prediction_enabled;
        }
        if let Some(prediction_time) = prediction_time {
            config.prediction_time = prediction_time;
        }
        if let Some(target_priority) = target_priority {
            config.target_priority = target_priority;
        }
        if let Some(ignore_frozen) = ignore_frozen {
            config.ignore_frozen = ignore_frozen;
        }

        Ok("Aimbot config updated".to_string())
    } else {
        Err("Not currently attached".to_string())
    }
}

#[tauri::command]
fn get_aimbot_status() -> Result<AimbotStatus, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();
        Ok(state.aimbot.get_status())
    } else {
        Err("Not currently attached".to_string())
    }
}

#[tauri::command]
fn register_trigger_key(key: String) -> Result<String, String> {
    if let Some(vk_code) = vk_code_from_string(&key) {
        InputHook::register_trigger_key(vk_code)?;
        Ok(format!("Trigger key '{}' registered", key))
    } else {
        Err(format!("Invalid key: {}", key))
    }
}

#[tauri::command]
fn unregister_trigger_key() -> Result<String, String> {
    InputHook::unregister_trigger_key()?;
    Ok("Trigger key unregistered".to_string())
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
            get_game_process_status,
            get_game_status,
            set_aimbot_enabled,
            set_aimbot_config,
            get_aimbot_status,
            register_trigger_key,
            unregister_trigger_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
