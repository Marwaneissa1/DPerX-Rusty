use crate::cheat_core::{Coords, Player};
use parking_lot::Mutex;
use std::sync::Arc;

const PHYS_SIZE: f32 = 28.0;
const FIRE_SPEED: f32 = 80.0;

#[derive(Clone, Debug)]
pub struct AimbotConfig {
    pub enabled: bool,
    pub fov: f32,
    pub silent: bool,
    pub hook_visible: bool,
    pub edge_scan: bool,
    pub max_distance: f32,
    pub always_active: bool,
    pub prediction_enabled: bool,
    pub prediction_time: f32,
    pub target_priority: String,
    pub ignore_frozen: bool,
    pub autofire: bool,
}

impl Default for AimbotConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            fov: 90.0,
            silent: false,
            hook_visible: false,
            edge_scan: true,
            max_distance: 395.0,
            always_active: false,
            prediction_enabled: true,
            prediction_time: 50.0,
            target_priority: "Closest".to_string(),
            ignore_frozen: true,
            autofire: false,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct AimbotStatus {
    pub enabled: bool,
    pub target_id: i32,
    pub target_visible: bool,
    pub target_pos: Coords,
    pub aim_pos: Coords,
}

pub struct Aimbot {
    config: Arc<Mutex<AimbotConfig>>,
    target_id: i32,
    target_visible: bool,
    target_pos: Coords,
    aim_pos: Coords,
}

impl Aimbot {
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(AimbotConfig::default())),
            target_id: -1,
            target_visible: false,
            target_pos: Coords { x: 0.0, y: 0.0 },
            aim_pos: Coords { x: 0.0, y: 0.0 },
        }
    }

    pub fn get_config(&self) -> Arc<Mutex<AimbotConfig>> {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: AimbotConfig) {
        *self.config.lock() = config;
    }

    pub fn get_status(&self) -> AimbotStatus {
        AimbotStatus {
            enabled: self.config.lock().enabled,
            target_id: self.target_id,
            target_visible: self.target_visible,
            target_pos: self.target_pos,
            aim_pos: self.aim_pos,
        }
    }

    pub fn update(
        &mut self,
        local_player_id: i32,
        players: &[Player],
        local_pos: Coords,
        local_vel: Coords,
        mouse_pos: Coords,
    ) -> Option<Coords> {
        let config: AimbotConfig = self.config.lock().clone();

        if !config.enabled {
            self.target_id = -1;
            self.target_visible = false;
            return None;
        }

        let target_id = self.get_closest_target(local_player_id, players, local_pos, mouse_pos, &config);

        if target_id == -1 {
            self.target_id = -1;
            self.target_visible = false;
            self.target_pos = Coords { x: 0.0, y: 0.0 };
            return None;
        }

        self.target_id = target_id;

        if let Some(target) = players.iter().find(|p| p.id == target_id) {
            let mut target_pos = target.pos;
            let target_vel = target.vel;

            self.predict_aim(local_pos, local_vel, &mut target_pos, target_vel);

            if config.edge_scan {
                target_pos = self.edge_scan(local_pos, target_pos);
            }

            self.target_pos = target_pos;
            self.aim_pos = self.normalize_aim(target_pos, local_pos);

            if self.aim_pos.x != 0.0 && self.aim_pos.y != 0.0 {
                return Some(self.aim_pos);
            }
        }

        None
    }

    fn get_closest_target(
        &self,
        local_id: i32,
        players: &[Player],
        local_pos: Coords,
        mouse_pos: Coords,
        config: &AimbotConfig,
    ) -> i32 {
        let mut closest_id = -1;
        let mut min_distance = config.max_distance;

        let mut filtered_players = players.to_vec();
        if config.ignore_frozen {
            filtered_players.retain(|p| !p.frozen);
        }

        for player in filtered_players {
            if player.id == local_id {
                continue;
            }

            if player.gametick == 0 {
                continue;
            }

            let distance = Self::distance(local_pos, player.pos);
            if distance > config.max_distance {
                continue;
            }

            let dir = Coords {
                x: player.pos.x - local_pos.x,
                y: player.pos.y - local_pos.y,
            };

            if !self.in_fov(config.fov, dir, mouse_pos) {
                continue;
            }

            if distance < min_distance {
                min_distance = distance;
                closest_id = player.id;
            }
        }

        closest_id
    }

    fn predict_aim(&self, my_pos: Coords, my_vel: Coords, target_pos: &mut Coords, target_vel: Coords) -> bool {
        let delta = Coords {
            x: target_pos.x - my_pos.x,
            y: target_pos.y - my_pos.y,
        };

        let delta_vel = Coords {
            x: target_vel.x - my_vel.x,
            y: target_vel.y - my_vel.y,
        };

        let target_speed = Self::length(target_vel);
        let fire_speed = target_speed + FIRE_SPEED;

        let a = Self::dot(delta_vel, delta_vel) - fire_speed * fire_speed;
        let b = 2.0 * Self::dot(delta_vel, delta);
        let c = Self::dot(delta, delta);

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let time = (2.0 * c / (discriminant.sqrt() - b)).abs() + 0.05;
            target_pos.x += target_vel.x * time;
            target_pos.y += target_vel.y * time;
            true
        } else {
            false
        }
    }

    fn edge_scan(&self, my_pos: Coords, target_pos: Coords) -> Coords {
        let dir = Coords {
            x: target_pos.x - my_pos.x,
            y: target_pos.y - my_pos.y,
        };

        let normalized = Self::normalize(dir);

        Coords {
            x: target_pos.x - normalized.x * (PHYS_SIZE * 0.5),
            y: target_pos.y - normalized.y * (PHYS_SIZE * 0.5),
        }
    }

    fn normalize_aim(&self, target_pos: Coords, local_pos: Coords) -> Coords {
        const CAMERA_MAX_DISTANCE: f32 = 200.0;
        const FOLLOW_FACTOR: f32 = 0.6;
        const DEAD_ZONE: f32 = 300.0;
        const MAX_DISTANCE: f32 = 300.0;

        let mouse_max = if FOLLOW_FACTOR != 0.0 {
            (CAMERA_MAX_DISTANCE / FOLLOW_FACTOR + DEAD_ZONE).min(MAX_DISTANCE)
        } else {
            MAX_DISTANCE
        };

        let mut pos = Coords {
            x: target_pos.x - local_pos.x,
            y: target_pos.y - local_pos.y,
        };

        let distance = Self::length(pos);
        if distance > 0.0 {
            pos.x = (pos.x / distance) * mouse_max;
            pos.y = (pos.y / distance) * mouse_max;
        }

        Coords {
            x: pos.x as i32 as f32,
            y: pos.y as i32 as f32,
        }
    }

    fn in_fov(&self, fov: f32, dir: Coords, mouse_pos: Coords) -> bool {
        let dir_angle = Self::angle(dir);
        let mouse_angle = Self::angle(mouse_pos);

        let mut diff = dir_angle - mouse_angle;

        while diff > std::f32::consts::PI {
            diff -= 2.0 * std::f32::consts::PI;
        }
        while diff < -std::f32::consts::PI {
            diff += 2.0 * std::f32::consts::PI;
        }

        let difference_angle = diff.abs() * 100.0;
        difference_angle <= fov
    }

    fn distance(a: Coords, b: Coords) -> f32 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn length(v: Coords) -> f32 {
        (v.x * v.x + v.y * v.y).sqrt()
    }

    fn normalize(v: Coords) -> Coords {
        let len = Self::length(v);
        if len > 0.0 {
            Coords {
                x: v.x / len,
                y: v.y / len,
            }
        } else {
            Coords { x: 0.0, y: 0.0 }
        }
    }

    fn dot(a: Coords, b: Coords) -> f32 {
        a.x * b.x + a.y * b.y
    }

    fn angle(v: Coords) -> f32 {
        v.y.atan2(v.x)
    }
}
