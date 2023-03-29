use bevy::{prelude::*, utils::HashMap};

mod events;
pub use events::*;

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


#[derive(Resource, Clone, Copy)]
pub struct Params {
    pub tile_size: f32,
    pub window_width: f32,
    pub window_height: f32,
}

#[derive(Resource)]
pub struct MaterialHandles {
    pub transparent: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub winner: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct TextureAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct TextureAtlasIndices {
    pub bg: usize,
    pub x: usize,
    pub o: usize,
    pub x_turn: usize,
    pub o_turn: usize,
    pub game_over_popup: usize,
}