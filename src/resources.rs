use bevy::prelude::*;

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
    pub bg_index: usize,
    pub x_index: usize,
    pub o_index: usize,
}