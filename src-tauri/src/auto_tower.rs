use crate::cheat_core::{Coords, Player};
use crate::input_hook::InputHook;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct AutoTowerConfig {
    pub enabled: bool,
    pub trigger_key: Option<i32>,
}

impl Default for AutoTowerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            trigger_key: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AutoTowerState {
    Idle,
    Positioning,
    Hooking,
}

pub struct AutoTower {
    config: Arc<Mutex<AutoTowerConfig>>,
    state: AutoTowerState,
    locked_target_id: Option<i32>,
    hook_start_time: Option<Instant>,
}

const TARGET_DISTANCE: f32 = 50.0;
const DISTANCE_TOLERANCE: f32 = 5.0;
const HOOK_DURATION: Duration = Duration::from_secs(3);

impl AutoTower {
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(AutoTowerConfig::default())),
            state: AutoTowerState::Idle,
            locked_target_id: None,
            hook_start_time: None,
        }
    }

    pub fn get_config(&self) -> Arc<Mutex<AutoTowerConfig>> {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: AutoTowerConfig) {
        *self.config.lock() = config;
    }

    pub fn update(&mut self, local_id: i32, local_pos: Coords, players: &[Player]) -> AutoTowerAction {
        let (enabled, trigger_key) = {
            let config = self.config.lock();
            (config.enabled, config.trigger_key)
        };

        if !enabled {
            self.reset();
            return AutoTowerAction::None;
        }

        let key_pressed = if let Some(key) = trigger_key {
            InputHook::check_key_state(key)
        } else {
            false
        };

        if !key_pressed {
            self.reset();
            return AutoTowerAction::None;
        }

        match self.state {
            AutoTowerState::Idle => {
                if let Some(target) = Self::find_nearest_player(local_id, local_pos, players) {
                    self.locked_target_id = Some(target.id);
                    self.state = AutoTowerState::Positioning;
                }
                AutoTowerAction::None
            }
            AutoTowerState::Positioning => {
                if let Some(target_id) = self.locked_target_id {
                    if let Some(target) = players.iter().find(|p| p.id == target_id && p.gametick > 0) {
                        let action = self.calculate_positioning(local_pos, target.pos);

                        if let AutoTowerAction::Hook { .. } = action {
                            self.state = AutoTowerState::Hooking;
                            self.hook_start_time = Some(Instant::now());
                        }

                        action
                    } else {
                        self.reset();
                        AutoTowerAction::None
                    }
                } else {
                    self.reset();
                    AutoTowerAction::None
                }
            }
            AutoTowerState::Hooking => {
                if let Some(start_time) = self.hook_start_time {
                    if start_time.elapsed() >= HOOK_DURATION {
                        self.reset();
                        return AutoTowerAction::None;
                    }
                }

                if let Some(target_id) = self.locked_target_id {
                    if let Some(target) = players.iter().find(|p| p.id == target_id && p.gametick > 0) {
                        return AutoTowerAction::Hook {
                            target_pos: target.pos,
                            should_fire: true,
                        };
                    }
                }

                self.reset();
                AutoTowerAction::None
            }
        }
    }

    fn calculate_positioning(&self, local_pos: Coords, target_pos: Coords) -> AutoTowerAction {
        let dx = target_pos.x - local_pos.x;
        let dy = target_pos.y - local_pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > TARGET_DISTANCE + DISTANCE_TOLERANCE {
            if dx > 0.0 {
                AutoTowerAction::MoveRight
            } else {
                AutoTowerAction::MoveLeft
            }
        } else if distance < TARGET_DISTANCE - DISTANCE_TOLERANCE {
            if dx > 0.0 {
                AutoTowerAction::MoveLeft
            } else {
                AutoTowerAction::MoveRight
            }
        } else {
            AutoTowerAction::Hook {
                target_pos,
                should_fire: true,
            }
        }
    }

    fn find_nearest_player(local_id: i32, local_pos: Coords, players: &[Player]) -> Option<&Player> {
        players
            .iter()
            .filter(|p| p.id != local_id && p.gametick > 0)
            .min_by(|a, b| {
                let dist_a = ((a.pos.x - local_pos.x).powi(2) + (a.pos.y - local_pos.y).powi(2)).sqrt();
                let dist_b = ((b.pos.x - local_pos.x).powi(2) + (b.pos.y - local_pos.y).powi(2)).sqrt();
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    fn reset(&mut self) {
        self.state = AutoTowerState::Idle;
        self.locked_target_id = None;
        self.hook_start_time = None;
    }

    pub fn get_state(&self) -> AutoTowerState {
        self.state
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AutoTowerAction {
    None,
    MoveLeft,
    MoveRight,
    Hook { target_pos: Coords, should_fire: bool },
}
