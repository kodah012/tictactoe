use bevy::{prelude::*, utils::HashMap, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};

use crate::data::*;

pub fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mat_handles: Res<MaterialHandles>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    params: Res<Params>,
) {
    let board_ent = commands.spawn(SpatialBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., -100.)),
        ..default()
    })
        .insert(Name::new("Board"))
        .id();
        
    let bg_ent = commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(tex_atlas_indices.bg),
            texture_atlas: tex_atlas_handle.0.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(8.))
                .with_translation(Vec3::new(0., 0., -100.)),
            ..default()
        },
        Name::new("Background"),
    )).id();
    commands.entity(board_ent).add_child(bg_ent);
    
    let mut board = Board(HashMap::new());
    
    for row in -1..=1 {
        for col in -1..=1 {
            let gap_multiplier = 1.18;
            let transform = Transform::from_scale(Vec3::splat(params.tile_size * 1.12))
                .with_translation(Vec3::new(
                    col as f32 * params.tile_size * gap_multiplier,
                    -(row as f32 * params.tile_size * gap_multiplier + 52.),
                    0.,
                ));
            let cell_pos = CellPosition { row, col };
            let cell_ent = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform,
                    material: mat_handles.transparent.clone_weak(),
                    ..default()
                },
                PickableBundle::default(),
                CellState::None,
                cell_pos,
                Name::new("Cell"),
            )).id();
            commands.entity(board_ent).add_child(cell_ent);
            
            board.0.insert(cell_pos, cell_ent);
        }
    }
    
    commands.insert_resource(board);
}

pub fn spawn_turn_text(
    mut commands: Commands,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
) {
    commands.spawn(SpriteSheetBundle {
        texture_atlas: tex_atlas_handle.0.clone_weak(),
        sprite: TextureAtlasSprite::new(tex_atlas_indices.x_turn),
        transform: Transform::from_scale(Vec3::splat(8.))
            .with_translation(Vec3::new(0., 270., -99.)),
        visibility: Visibility::Hidden,
        ..default()
    })
        .insert(TurnText)
        .insert(Name::new("Turn Text"));
}

pub fn spawn_game_over_popup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    mat_handles: Res<MaterialHandles>,
) {
    let o_text_sprite_ent = commands.spawn(SpriteSheetBundle {
        texture_atlas: tex_atlas_handle.0.clone_weak(),
        sprite: TextureAtlasSprite::new(tex_atlas_indices.o_text),
        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
        ..default()
    })
        .insert(Name::new("O Text"))
        .id();
    
    let o_text_bg_sprite_ent = commands.spawn(SpriteSheetBundle {
        texture_atlas: tex_atlas_handle.0.clone_weak(),
        sprite: TextureAtlasSprite::new(tex_atlas_indices.o_text_bg),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    })
        .insert(Name::new("O Text Background"))
        .id();
    
    let o_text_ent = commands.spawn(SpatialBundle {
        transform: Transform::from_translation(Vec3::new(-10., 15.5, 1.)),
        ..default()
    })
        .insert(mat_handles.bg.clone_weak())
        .add_child(o_text_sprite_ent)
        .add_child(o_text_bg_sprite_ent)
        .id();

    let play_btn_ent = commands.spawn(SpriteSheetBundle {
        texture_atlas: tex_atlas_handle.0.clone_weak(),
        sprite: TextureAtlasSprite::new(tex_atlas_indices.play_btn),
        transform: Transform::from_translation(Vec3::new(0., 4.5, 1.)),
        ..default()
    })
        .insert(PlayBtn)
        .insert(PickableBundle::default())
        .insert(Name::new("Play Button"))
        .id();
    
    let quit_btn_ent = commands.spawn(SpriteSheetBundle {
        texture_atlas: tex_atlas_handle.0.clone_weak(),
        sprite: TextureAtlasSprite::new(tex_atlas_indices.quit_btn),
        transform: Transform::from_translation(Vec3::new(0., -4.5, 1.)),
        ..default()
    })
        .insert(Mesh2dHandle(meshes.add(Mesh::from(shape::Quad::default())).into()))
        .insert(QuitBtn)
        .insert(PickableBundle::default())
        .insert(Name::new("Quit Button"))
        .id();

    commands.spawn(SpriteSheetBundle {
        texture_atlas: tex_atlas_handle.0.clone_weak(),
        sprite: TextureAtlasSprite::new(tex_atlas_indices.game_over_popup),
        transform: Transform::from_scale(Vec3::splat(8.))
            .with_translation(Vec3::new(0., 0., -99.)),
        visibility: Visibility::Hidden,
        ..default()
    })
        .insert(GameOverPopup::X)
        .insert(Name::new("Game Over Popup"))
        .add_child(o_text_ent)
        .add_child(play_btn_ent)
        .add_child(quit_btn_ent);
}

pub fn init_textures(
    mut commands: Commands,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let tex_handle = asset_server.load("../assets/atlas.png");
    let mut tex_atlas = TextureAtlas::new_empty(tex_handle, Vec2::new(248., 119.));

    let bg = tex_atlas.add_texture(Rect {
        min: Vec2::new(125., 3.),
        max: Vec2::new(189., 116.),
    });
    let x = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 97.),
        max: Vec2::new(208., 113.),
    });
    let o = tex_atlas.add_texture(Rect {
        min: Vec2::new(211., 97.),
        max: Vec2::new(227., 113.),
    });
    let x_turn = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 82.),
        max: Vec2::new(218., 87.),
    });
    let o_turn = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 89.),
        max: Vec2::new(218., 94.),
    });
    let game_over_popup= tex_atlas.add_texture(Rect {
        min: Vec2::new(191., 35.),
        max: Vec2::new(245., 75.),
    });
    let o_text = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 89.),
        max: Vec2::new(196., 94.),
    });
    let o_text_bg = tex_atlas.add_texture(Rect {
        min: Vec2::new(202., 37.),
        max: Vec2::new(206., 42.)
    });
    let play_btn = tex_atlas.add_texture(Rect {
        min: Vec2::new(191., 17.),
        max: Vec2::new(216., 24.),
    });
    let quit_btn = tex_atlas.add_texture(Rect {
        min: Vec2::new(191., 27.),
        max: Vec2::new(216., 34.),
    });
    commands.insert_resource(TextureAtlasIndices {
        bg,
        x,
        o,
        x_turn,
        o_turn,
        game_over_popup,
        o_text,
        o_text_bg,
        play_btn,
        quit_btn,
    });

    let tex_atlas_handle = tex_atlases.add(tex_atlas);
    commands.insert_resource(TextureAtlasHandle(tex_atlas_handle));
}

pub fn init_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transparent = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.),
        ..default()
    });
    
    let hovered = materials.add(ColorMaterial {
        color: Color::hex("#6540537f").unwrap(),
        ..default()
    });
    
    let winner = materials.add(ColorMaterial {
        color: Color::hex("#654053").unwrap(),
        ..default()
    });

    let bg = materials.add(ColorMaterial {
        color: Color::hex("#654053").unwrap(),
        ..default()
    });
    
    commands.insert_resource(MaterialHandles {
        transparent,
        hovered,
        winner,
        bg,
    });
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        ..default()
    })
        .insert(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
}
