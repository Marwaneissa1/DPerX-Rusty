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
    pub direction: usize,
    pub jump: usize,
    pub fire: usize,
    pub hook: usize,
    pub game_status: usize,
    pub left_dir: usize,
    pub right_dir: usize,
    pub hook_line: usize,
}

#[allow(dead_code)]
pub struct ServerOffsets {
    // pub world_ptr_chain: [usize; 12],
    pub local_player_id: usize,
    pub online_players: usize,

    pub player_gametick: usize,
    pub player_fixed_coords: usize,
    pub player_vel: usize,
    pub player_aim_angle: usize,
    pub player_movement_dir: usize,
    pub player_jump: usize,
    pub player_hook: usize,
    pub player_time_hooked: usize,
    pub player_last_hook_coords: usize,
    pub player_game_status: usize,
    pub player_frozen_time: usize,
    pub player_selected_weapon: usize,
    pub player_frozen: usize,
    pub player_last_tick_fired: usize,
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
        server: 0x615760,
        client: 0x5E74D0,
    },
    client_offsets: ClientOffsets {
        aim_pos_screen: CoordOffset::new(0x10),
        aim_pos_screen_dummy: CoordOffset::new(0x18),
        aim_pos_world: CoordOffset::new(0x30),
        gametick: 0x58,
        direction: 0x60,
        jump: 0x6C,
        fire: 0x70,
        hook: 0x74,
        game_status: 0x78,
        left_dir: 0x100,
        right_dir: 0x108,
        hook_line: 0x110,
    },
    server_offsets: ServerOffsets {
        local_player_id: 0x20A0,
        online_players: 0x20A4,

        player_gametick: 0x20E4,
        player_fixed_coords: 0x20E8,
        player_vel: 0x20F0,
        player_aim_angle: 0x20F8,
        player_movement_dir: 0x20FC,
        player_jump: 0x2100,
        player_hook: 0x2108,
        player_time_hooked: 0x210C,
        player_last_hook_coords: 0x2118,
        player_game_status: 0x2120,
        player_frozen_time: 0x2128,
        player_selected_weapon: 0x2130,
        player_frozen: 0x2134,
        player_last_tick_fired: 0x2135,
        player_pos: 0x21CC,
    },
};
