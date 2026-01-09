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
    pub world_ptr_chain: [usize; 13],
    pub local_player_id: usize,
    pub online_players: usize,
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
        world_ptr_chain: [
            0x338, 0x100, 0x8, 0x1118, 0xCD8, 0x18, 0x320, 0x10, 0xE8, 0x20, 0x8, 0x128, 0x0
        ],
        local_player_id: 0x64E8,
        online_players: 0x64EC
    },
};
