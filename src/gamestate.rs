pub struct GameState {
    player_char: char,
    pub(crate) player_x: u32,
    pub(crate) player_y: u32,
}

trait Command {
    fn new() -> Self;
}

impl GameState {
    pub fn init() -> GameState {
        GameState {
            player_char: '@',
            player_x: 10,
            player_y: 10,
        }
    }
}