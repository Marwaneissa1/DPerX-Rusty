use crate::game_structure::OFFSETS;
use crate::ioprocesses::Process;

// pub fn test() {
    
//     let aim_screen_x = match process.read::<f32>(client_ptr + OFFSETS.client_offsets.aim_pos_screen.x()) {
//         Ok(v) => v,
//         Err(e) => { 
//             eprintln!("Error reading local player aim screen x: {}", e);
//             return;
//         }
//     };

//     let aim_screen_y = match process.read::<f32>(client_ptr + OFFSETS.client_offsets.aim_pos_screen.y()) {
//         Ok(v) => v,
//         Err(e) => {
//             eprintln!("Error reading local player aim screen y: {}", e);
//             return;
//         } 
//     };

//     let aim_world_x = match process.read::<f32>(client_ptr + OFFSETS.client_offsets.aim_pos_world.x()) { 
//         Ok(v) => v,
//         Err(e) => {
//             eprintln!("Error reading local player aim world x: {}", e);
//             return;
//         }
//     };
//     let aim_world_y = match process.read::<f32>(client_ptr + OFFSETS.client_offsets.aim_pos_world.y()) {
//         Ok(v) => v,
//         Err(e) => {
//             eprintln!("Error reading local player aim world y: {}", e);
//             return;
//         }
//     };

//     let _ = process.write(client_ptr + OFFSETS.client_offsets.aim_pos_screen.x(), &0i32);
//     let _ = process.write(client_ptr + OFFSETS.client_offsets.aim_pos_screen.y(), &0i32);

// }

pub fn attach() {    
    let process = match Process::from_process_name("ddnet.exe") {
        Ok(p) => {
            println!("Found process with PID: {}", p.pid());
            println!("Module base address: 0x{:X}", p.base_address());
            p
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let client_ptr = match process.read::<usize>(process.base_address() + OFFSETS.base.client) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading client base: {}", e);
            return;
        }
    };

    let server_ptr = match process.read::<usize>(process.base_address() + OFFSETS.base.server) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error reading server base: {}", e);
            return;
        }
    };

    let local_player_id = {
        let mut current_address = server_ptr;

        for &offset in &OFFSETS.server_offsets.world_ptr_chain {
            current_address = match process.read::<usize>(current_address + offset) {
                Ok(addr) => addr,
                Err(e) => {
                    eprintln!("Error following pointer chain at offset 0x{:X}: {}", offset, e);
                    return;
                }
            };
        }

        println!("Current address: 0x{:X}", current_address);

        match process.read::<i32>(current_address + OFFSETS.server_offsets.local_player_id) {
            Ok(id) => {
                println!("Local Player ID: {}", id);
                id
            }
            Err(e) => {
                eprintln!("Error reading local player ID: {}", e);
                return;
            }
        }
    };

}