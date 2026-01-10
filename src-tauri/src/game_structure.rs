#[derive(Clone, Copy)]
pub struct CoordOffset {
    base: usize,
}

impl CoordOffset {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    pub const fn x(&self) -> usize {
        self.base
    }

    pub const fn y(&self) -> usize {
        self.base + 4
    }
}

#[allow(dead_code)]
pub struct ClientOffsets {
    pub aim_pos_screen: CoordOffset,
    pub aim_pos_screen_dummy: CoordOffset,
    pub aim_pos_world: CoordOffset,
    pub gametick: usize,
}

#[allow(dead_code)]
pub struct ServerOffsets {
    pub world_ptr_chain: [usize; 12],
    pub local_player_id: usize,
    pub online_players: usize,

    pub player_gametick: usize,
    pub player_fixed_coords: usize,
    pub player_vel: usize,
    pub player_sim_angle: usize,
    pub player_current_weapon: usize,
    pub player_jump_status: usize,
    pub player_hook: usize,
    pub player_time_hooked: usize,
    pub player_selected_weapon: usize,
    pub player_pos: usize,
}

#[allow(dead_code)]
pub struct BaseOffsets {
    pub server: usize,
    pub client: usize,
}

pub struct Offsets {
    pub base: BaseOffsets,
    pub client_offsets: ClientOffsets,
    pub server_offsets: ServerOffsets,
}

pub static OFFSETS: Offsets = Offsets {
    base: BaseOffsets {
        server: 0x629C40,
        client: 0x598AB8,
    },
    client_offsets: ClientOffsets {
        aim_pos_screen: CoordOffset::new(0x10),
        aim_pos_screen_dummy: CoordOffset::new(0x18),
        aim_pos_world: CoordOffset::new(0x28),
        gametick: 0x30,
    },
    server_offsets: ServerOffsets {
        world_ptr_chain: [0x90, 0x140, 0xD0, 0x10, 0xE8, 0x20, 0x10, 0xC8, 0x138, 0x8, 0x0, 0x0],
        local_player_id: 0x64E8,
        online_players: 0x64EC,

        player_gametick: 0x652C,
        player_fixed_coords: 0x6530,
        player_vel: 0x6538,
        player_sim_angle: 0x6544,
        player_current_weapon: 0x6548,
        player_jump_status: 0x654C,
        player_hook: 0x6558,
        player_time_hooked: 0x655C,
        player_selected_weapon: 0x660C,
        player_pos: 0x6610,
    },
};
