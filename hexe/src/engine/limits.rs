pub struct Limits {
    pub ponder: bool,
    pub infinite: bool,
    pub moves_to_go: u32,
    pub time: [u32; 2],
    pub inc: [u32; 2],
    pub depth: u32,
    pub nodes: u32,
    pub mate: u32,
    pub move_time: u32,
}
