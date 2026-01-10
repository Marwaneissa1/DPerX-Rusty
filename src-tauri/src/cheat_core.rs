use crate::game_structure::OFFSETS;
use crate::ioprocesses::Process;

#[derive(Clone, Copy, Default, serde::Serialize)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

pub struct CheatCore {
    process: Process,
    client_ptr: usize,
    server_ptr: usize,
    world_ptr: usize,
    local_player_id: i32,
    online_players: i32,
    aim_screen: Coords,
    aim_world: Coords,
    player_pos: Coords,
}

impl CheatCore {
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
            world_ptr: 0,
            local_player_id: 0,
            online_players: 0,
            aim_screen: Coords { x: 0.0, y: 0.0 },
            aim_world: Coords { x: 0.0, y: 0.0 },
            player_pos: Coords { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self) {
        for &offset in &OFFSETS.server_offsets.world_ptr_chain {
            self.server_ptr = match self.process.read::<usize>(self.server_ptr + offset) {
                Ok(addr) => addr,
                Err(e) => {
                    eprintln!("Error following pointer chain at offset 0x{:X}: {}", offset, e);
                    return;
                }
            };
        }


        self.local_player_id = match self
            .process
            .read::<i32>(self.server_ptr + OFFSETS.server_offsets.local_player_id)
        {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Error reading local player ID: {}", e);
                return;
            }
        };

        self.online_players = match self
            .process
            .read::<i32>(self.server_ptr + OFFSETS.server_offsets.online_players)
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error reading online players: {}", e);
                return;
            }
        };

        self.aim_screen.x = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.x())
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error reading local player aim screen x: {}", e);
                return;
            }
        };

        self.aim_screen.y = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.y())
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error reading local player aim screen y: {}", e);
                return;
            }
        };

        self.aim_world.x = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_world.x())
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error reading local player aim world x: {}", e);
                return;
            }
        };

        self.aim_world.y = match self
            .process
            .read::<f32>(self.client_ptr + OFFSETS.client_offsets.aim_pos_world.y())
        {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error reading local player aim world y: {}", e);
                return;
            }
        };

        // let _ = self.process.write(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.x(), &0i32);
        // let _ = self.process.write(self.client_ptr + OFFSETS.client_offsets.aim_pos_screen.y(), &0i32);
    }

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
}
