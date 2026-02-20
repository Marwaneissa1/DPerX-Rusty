mod aimbot;
mod auto_tower;
mod balancer;
mod cheat_core;
mod input_hook;

mod ioprocesses;
mod offsets;

use aimbot::{Aimbot, AimbotStatus};
use auto_tower::{AutoTower, AutoTowerAction};
use balancer::{Balancer, MovementDirection};
use cheat_core::{CheatCore, Coords};
use input_hook::{vk_code_from_string, InputHook};
use parking_lot::{Mutex, RwLock};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct AppState {
    cheat_core: Option<CheatCore>,
    aimbot: Aimbot,
    balancer: Balancer,
    auto_tower: AutoTower,
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
    frozen: bool,
}

#[tauri::command]
fn attach() -> Result<String, String> {
    let mut global = GLOBAL_STATE.write();

    if global.is_none() {
        let state = Arc::new(Mutex::new(AppState {
            cheat_core: None,
            aimbot: Aimbot::new(),
            balancer: Balancer::new(),
            auto_tower: AutoTower::new(),
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

                let config = state.aimbot.get_config().lock().clone();

                if config.autofire {
                    if let Some(ref core) = state.cheat_core {
                        let _ = core.shoot();
                    }
                }

                if should_update {
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

                let movement_dir = state.balancer.update(local_id, local_pos, &players);
                // let auto_tower_action = state.auto_tower.update(local_id, local_pos, &players);

                // if let Some(ref core) = state.cheat_core {
                //     match auto_tower_action {
                //         AutoTowerAction::MoveLeft => {
                //             let _ = core.write_movement(true, false);
                //             let _ = core.write_hook(false);
                //         }
                //         AutoTowerAction::MoveRight => {
                //             let _ = core.write_movement(false, true);
                //             let _ = core.write_hook(false);
                //         }
                //         AutoTowerAction::Hook {
                //             target_pos,
                //             should_fire,
                //         } => {
                //             let _ = core.write_movement(false, false);
                //             let _ = core.write_aim_position(target_pos);
                //             let _ = core.write_hook(true);
                //             if should_fire {
                //                 let _ = core.shoot();
                //             }
                //         }
                //         AutoTowerAction::None => {
                //             match movement_dir {
                //                 MovementDirection::Left => {
                //                     let _ = core.write_movement(true, false);
                //                 }
                //                 MovementDirection::Right => {
                //                     let _ = core.write_movement(false, true);
                //                 }
                //                 MovementDirection::None => {
                //                     let _ = core.write_movement(false, false);
                //                 }
                //             }
                //             let _ = core.write_hook(false);
                //         }
                //     }
                // }
            }
        });
    }

    let state_arc = global.as_ref().unwrap().clone();
    drop(global);

    let mut state = state_arc.lock();

    let core = match CheatCore::new() {
        Ok(c) => c,
        Err(e) => {
            state.is_attached = false;
            return Err(format!("Failed to attach: {}", e));
        }
    };

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
                // .filter(|p| p.gametick > 0)
                .map(|p| PlayerInfo {
                    id: p.id,
                    gametick: p.gametick,
                    pos: p.pos,
                    vel: p.vel,
                    frozen: p.frozen,
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
    autofire: Option<bool>,
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
        if let Some(autofire) = autofire {
            config.autofire = autofire;
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

#[tauri::command]
fn set_balancer_enabled(enabled: bool) -> Result<String, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        let mut config = state.balancer.get_config().lock().clone();
        config.enabled = enabled;
        state.balancer.set_config(config);
        Ok(format!("Balancer {}", if enabled { "enabled" } else { "disabled" }))
    } else {
        Err("Not currently attached".to_string())
    }
}

#[derive(serde::Serialize)]
struct BalancerStatus {
    enabled: bool,
}

#[tauri::command]
fn get_balancer_status() -> Result<BalancerStatus, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();
        let config_guard = state.balancer.get_config();
        let config = config_guard.lock();
        Ok(BalancerStatus {
            enabled: config.enabled,
        })
    } else {
        Err("Not currently attached".to_string())
    }
}

#[tauri::command]
fn set_auto_tower_enabled(enabled: bool) -> Result<String, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let mut state = state_arc.lock();
        let mut config = state.auto_tower.get_config().lock().clone();
        config.enabled = enabled;
        state.auto_tower.set_config(config);
        Ok(format!("Auto Tower {}", if enabled { "enabled" } else { "disabled" }))
    } else {
        Err("Not currently attached".to_string())
    }
}

#[tauri::command]
fn set_auto_tower_key(key: String) -> Result<String, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        if let Some(vk_code) = vk_code_from_string(&key) {
            let mut state = state_arc.lock();
            let mut config = state.auto_tower.get_config().lock().clone();
            config.trigger_key = Some(vk_code);
            state.auto_tower.set_config(config);
            Ok(format!("Auto Tower key set to '{}'", key))
        } else {
            Err(format!("Invalid key: {}", key))
        }
    } else {
        Err("Not currently attached".to_string())
    }
}

#[derive(serde::Serialize)]
struct AutoTowerStatus {
    enabled: bool,
    trigger_key: Option<String>,
}

#[tauri::command]
fn get_auto_tower_status() -> Result<AutoTowerStatus, String> {
    let global = GLOBAL_STATE.read();

    if let Some(state_arc) = global.as_ref() {
        let state = state_arc.lock();
        let config_guard = state.auto_tower.get_config();
        let config = config_guard.lock();

        let key_string = config.trigger_key.map(|vk| format!("{}", vk));

        Ok(AutoTowerStatus {
            enabled: config.enabled,
            trigger_key: key_string,
        })
    } else {
        Err("Not currently attached".to_string())
    }
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    if !src.exists() {
        return Err(format!("Source folder does not exist: {:?}", src));
    }

    fs::create_dir_all(dst).map_err(|e| format!("Failed to create destination folder: {}", e))?;

    for entry in fs::read_dir(src).map_err(|e| format!("Failed to read source directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let ty = entry
            .file_type()
            .map_err(|e| format!("Failed to get file type: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }

    Ok(())
}

fn get_desktop_path() -> Result<PathBuf, String> {
    let userprofile = std::env::var("USERPROFILE").map_err(|_| "Failed to get USERPROFILE")?;
    Ok(PathBuf::from(userprofile).join("Desktop"))
}

#[tauri::command]
fn execute_local_spoofer() -> Result<String, String> {
    use ioprocesses::Process;

    if Process::from_process_name("ddnet.exe").is_ok() {
        return Err("Cannot execute Local Spoofer while DDNet is running. Please close the game first.".to_string());
    }

    let appdata = std::env::var("APPDATA").map_err(|_| "Failed to get APPDATA path")?;
    let desktop = get_desktop_path()?;

    let mut backed_up = Vec::new();
    let mut errors = Vec::new();

    let ddnet_src = PathBuf::from(&appdata).join("DDNet");
    let ddnet_dst = desktop.join("DDNet_backup");

    if ddnet_src.exists() {
        match copy_dir_all(&ddnet_src, &ddnet_dst) {
            Ok(_) => match fs::remove_dir_all(&ddnet_src) {
                Ok(_) => backed_up.push("DDNet"),
                Err(e) => errors.push(format!("Failed to delete DDNet from AppData: {}", e)),
            },
            Err(e) => errors.push(format!("Failed to backup DDNet: {}", e)),
        }
    }

    let teeworlds_src = PathBuf::from(&appdata).join("TeeWorlds");
    let teeworlds_dst = desktop.join("TeeWorlds_backup");

    if teeworlds_src.exists() {
        match copy_dir_all(&teeworlds_src, &teeworlds_dst) {
            Ok(_) => match fs::remove_dir_all(&teeworlds_src) {
                Ok(_) => backed_up.push("TeeWorlds"),
                Err(e) => errors.push(format!("Failed to delete TeeWorlds from AppData: {}", e)),
            },
            Err(e) => errors.push(format!("Failed to backup TeeWorlds: {}", e)),
        }
    }

    if backed_up.is_empty() && errors.is_empty() {
        return Err("No folders found to backup (DDNet and TeeWorlds not found in AppData)".to_string());
    }

    if !errors.is_empty() {
        return Err(format!("Errors occurred: {}", errors.join("; ")));
    }

    Ok(format!("Successfully backed up and removed: {}", backed_up.join(", ")))
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
            set_balancer_enabled,
            get_balancer_status,
            set_auto_tower_enabled,
            set_auto_tower_key,
            get_auto_tower_status,
            execute_local_spoofer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
