use crate::cheat_core::{Coords, Player};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct BalancerConfig {
    pub enabled: bool,
}

impl Default for BalancerConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MovementDirection {
    Left,
    Right,
    None,
}

pub struct Balancer {
    config: Arc<Mutex<BalancerConfig>>,
}

impl Balancer {
    pub fn new() -> Self {
        Self {
            config: Arc::new(Mutex::new(BalancerConfig::default())),
        }
    }

    pub fn get_config(&self) -> Arc<Mutex<BalancerConfig>> {
        self.config.clone()
    }

    pub fn set_config(&mut self, config: BalancerConfig) {
        *self.config.lock() = config;
    }

    pub fn update(&self, local_id: i32, local_pos: Coords, players: &[Player]) -> MovementDirection {
        let config = self.config.lock();
        if !config.enabled {
            return MovementDirection::None;
        }
        drop(config);

        if let Some(nearest) = Self::find_nearest_player(local_id, local_pos, players) {
            if local_pos.x > nearest.pos.x {
                MovementDirection::Left
            } else if local_pos.x < nearest.pos.x {
                MovementDirection::Right
            } else {
                MovementDirection::None
            }
        } else {
            MovementDirection::None
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
}
