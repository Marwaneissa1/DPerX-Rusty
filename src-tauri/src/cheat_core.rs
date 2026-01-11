use crate::ioprocesses::Process;
use crate::offsets::OFFSETS;
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Clone, Copy, Default, serde::Serialize, Debug)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Default, serde::Serialize)]
pub struct Player {
    pub id: i32,
    pub gametick: i64,
    pub pos: Coords,
    pub vel: Coords,
    pub frozen: bool,
}

pub struct CheatCore {
    process: Process,
    client_ptr: usize,
    server_ptr: usize,
    local_player_id: i32,
    online_players: i32,
    aim_screen: Coords,
    aim_world: Coords,
    player_pos: Coords,
    player_vel: Coords,
    shoot_index: AtomicI32,
    players: Vec<Player>,
}

impl CheatCore {
    pub fn get_local_player_id(&self) -> i32 {
        self.local_player_id
    }

    pub fn get_online_players(&self) -> i32 {
        self.online_players
    }

    pub fn get_player_pos(&self) -> Coords {
        self.player_pos
    }

    pub fn get_aim_screen(&self) -> Coords {
        self.aim_screen
    }

    pub fn get_aim_world(&self) -> Coords {
        self.aim_world
    }

    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn get_local_velocity(&self) -> Coords {
        self.player_vel
    }

    pub fn new() -> Self {
        let process = match Process::from_process_name("ddnet.exe") {
            Ok(p) => {
                println!("Found process with PID: {}", p.pid());
                println!("Module base address: 0x{:X}", p.base_address());
                p
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Error: {}", e);
            }
        };

        let client_ptr = match process.read::<usize>(process.base_address() + OFFSETS.base.client) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error reading client base: {}", e);
                panic!("Error reading client base: {}", e);
            }
        };

        let server_ptr = match process.read::<usize>(process.base_address() + OFFSETS.base.server) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error reading server base: {}", e);
                panic!("Error reading server base: {}", e);
            }
        };

        Self {
            process,
            client_ptr,
            server_ptr,
            local_player_id: 0,
            online_players: 0,
            aim_screen: Coords { x: 0.0, y: 0.0 },
            aim_world: Coords { x: 0.0, y: 0.0 },
            player_pos: Coords { x: 0.0, y: 0.0 },
            player_vel: Coords { x: 0.0, y: 0.0 },
            shoot_index: AtomicI32::new(0),
            players: Vec::new(),
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        // update base data
        self.local_player_id = match self
            .process
            .read::<i32>(self.server_ptr + OFFSETS.server_offsets.local_player_id)
        {
            Ok(id) => id,
            Err(e) => {
                return Err(format!("Error reading local player ID: {}", e));
            }
        };

        self.online_players = match self
            .process
            .read::<i32>(self.server_ptr + OFFSETS.server_offsets.online_players)
        {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error reading online players: {}", e));
            }
        };

        self.aim_screen.x = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.x())
        {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error reading aim screen x: {}", e));
            }
        };

        self.aim_screen.y = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.y())
        {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error reading aim screen y: {}", e));
            }
        };

        self.aim_world.x = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_world.x())
        {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error reading aim world x: {}", e));
            }
        };

        self.aim_world.y = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_world.y())
        {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Error reading aim world y: {}", e));
            }
        };

        // get players in server
        self.players.clear();
        for i in 0..64 {
            let offset = i * 0xF8;

            let gametick = match self
                .process
                .read::<i64>(self.server_ptr + OFFSETS.server_offsets.player_gametick + offset)
            {
                Ok(p) => p,
                Err(e) => {
                    return Err(format!("Error reading player gametick: {}", e));
                }
            };

            let pos = match self
                .process
                .read::<Coords>(self.server_ptr + OFFSETS.server_offsets.player_pos + offset)
            {
                Ok(pos) => pos,
                Err(e) => {
                    return Err(format!("Error reading player pos: {}", e));
                }
            };

            let vel = match self
                .process
                .read::<Coords>(self.server_ptr + OFFSETS.server_offsets.player_vel + offset)
            {
                Ok(vel) => vel,
                Err(e) => {
                    return Err(format!("Error reading player vel: {}", e));
                }
            };

            let frozen = match self
                .process
                .read::<bool>(self.server_ptr + OFFSETS.server_offsets.player_frozen + offset)
            {
                Ok(frozen) => frozen,
                Err(e) => {
                    return Err(format!("Error reading player frozen: {}", e));
                }
            };

            self.players.push(Player {
                id: i as i32,
                gametick,
                pos,
                vel,
                frozen,
            });
        }

        let local_player: Result<&Player, String> = match self.players.get(self.local_player_id as usize) {
            Some(player) => Ok(player),
            None => Err("Local player not found".to_string()),
        };

        if let Ok(p) = local_player {
            self.player_pos = p.pos;
            self.player_vel = p.vel;
        }

        Ok(())
    }

    pub fn write_aim_position(&self, pos: Coords) -> Result<(), String> {
        self.process
            .write(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.x(), &pos.x)
            .map_err(|e| format!("Failed to write aim x: {}", e))?;

        self.process
            .write(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.y(), &pos.y)
            .map_err(|e| format!("Failed to write aim y: {}", e))?;

        Ok(())
    }

    pub fn shoot(&self) -> Result<(), String> {
        let index = self.shoot_index.fetch_add(1, Ordering::SeqCst);
        let new_index = index + 1;

        self.process
            .write(self.client_ptr + OFFSETS.client_offsets.fire, &new_index)
            .map_err(|e| format!("Failed to shoot: {}", e))?;

        Ok(())
    }
}
