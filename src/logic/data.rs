use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct Board(pub HashMap<CellPosition, Entity>);

#[derive(Eq, PartialEq, Debug, States, Hash, Default, Clone)]
pub enum GameState {
    #[default]
    XTurn,
    OTurn,
    GameOver,
}

#[derive(Component, Reflect, Eq, PartialEq, Clone, Copy, Debug)]
pub enum CellState {
    None,
    X,
    O,
}

#[derive(Component, Reflect, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct CellPosition {
    pub row: i32,
    pub col: i32,
}

pub struct GameOverEvent(pub [CellPosition; 3]);