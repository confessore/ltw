#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Default,
    Playing
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PlayerState {
    Default,
    Menu
}
