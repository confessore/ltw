#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Default,
    Playing,
    Menu
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlayerState {
    Default,
    Playing,
    Menu
}
