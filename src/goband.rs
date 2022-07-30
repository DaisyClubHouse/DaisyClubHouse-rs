struct GobandManager {
    rooms: Vec<GobandRoom>,
}

impl GobandManager {
    fn new() -> GobandManager {
        GobandManager { rooms: Vec::new() }
    }

    fn add_room(&mut self, room: GobandRoom) {
        self.rooms.push(room);
    }
}

struct GobandRoom {
    id: u32,               // 房间ID
    name: String,          // 房间名称
    game_cfg: GobandConfig, // 游戏设置
}

struct GobandConfig {
    room_count: usize,
}
