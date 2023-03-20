use bevy::{prelude::*, utils::HashMap, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, PickingEvent, HoverEvent};
use crate::resources::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameState::XTurn)
            .add_startup_system(spawn_board)
            .add_system_to_stage(CoreStage::PostUpdate, handle_hover)
            .add_system_to_stage(CoreStage::PostUpdate, handle_picking)
            .register_type::<CellState>()
            .register_type::<CellPosition>();
    }
}

#[derive(Resource)]
struct Board(HashMap<CellPosition, Entity>);

#[derive(Resource, Eq, PartialEq, Debug)]
enum GameState {
    XTurn,
    OTurn,
    GameOver,
}

#[derive(Component, Reflect, Eq, PartialEq, Clone, Copy, Debug)]
enum CellState {
    None,
    X,
    O,
}

#[derive(Component, Reflect, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct CellPosition {
    row: i32,
    col: i32,
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mat_handles: Res<MaterialHandles>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    params: Res<Params>,
) {
    let board_ent = commands.spawn(SpatialBundle::default())
        .insert(Name::new("Board"))
        .id();
        
    let bg_ent = commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(tex_atlas_indices.bg_index),
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

fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut game_state: ResMut<GameState>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    cell_qry: Query<(&CellState, &CellPosition)>,
    board: Res<Board>,
) {
    events.iter().for_each(|event| {
        match event {
            PickingEvent::Clicked(ent) => {
                let (state, cell_pos) = cell_qry.get(*ent).unwrap();
                if *state == CellState::None {
                    let sprite_index = if *game_state == GameState::XTurn { tex_atlas_indices.x_index } else { tex_atlas_indices.o_index };
                    let cell_state = if *game_state == GameState::XTurn { CellState::X } else { CellState::O };
                    
                    let sprite_ent = commands.spawn(SpriteSheetBundle {
                        texture_atlas: tex_atlas_handle.0.clone_weak(),
                        sprite: TextureAtlasSprite::new(sprite_index),
                        transform: Transform::from_scale(Vec3::splat(0.05)),
                        ..default()
                    }).id();

                    commands.entity(*ent)
                        .insert(cell_state)
                        .add_child(sprite_ent);

                    let game_over = check_game_over(&cell_qry, (&cell_state, cell_pos), &board);
                    if game_over {
                        handle_game_over(&mut game_state);
                    } else {
                        let new_state = if *game_state == GameState::XTurn { GameState::OTurn } else { GameState::XTurn };
                        *game_state = new_state;
                    }
                }
            },
            _ => (),
        }
    });
}

fn handle_hover(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mat_handles: Res<MaterialHandles>,
) {
    events.iter().for_each(|event| {
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(ent)) => {
                commands.entity(*ent).insert(mat_handles.hovered.clone_weak());
            },
            PickingEvent::Hover(HoverEvent::JustLeft(ent)) => {
                commands.entity(*ent).insert(mat_handles.transparent.clone_weak());
            },
            _ => (),
        }
    });
}

fn check_game_over(
    cell_qry: &Query<(&CellState, &CellPosition)>,
    picked_cell: (&CellState, &CellPosition),
    board: &Board,
) -> bool {
    let picked_state = *picked_cell.0;
    let picked_pos = *picked_cell.1;

    // Check horizontal
    for col in -1..=1 {
        if col == picked_pos.col {
            if col == 1 {
                return true;
            }
            continue;
        }

        let pos = CellPosition { row: picked_pos.row, col };
        let ent = board.0.get(&pos).unwrap();
        let (state, _) = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if col == 1 {
            return true;
        }
    }
    
    // Check vertical
    for row in -1..=1 {
        if row == picked_pos.row {
            if row == 1 {
                return true;
            }
            continue;
        }

        let pos = CellPosition { row, col: picked_pos.col };
        let ent = board.0.get(&pos).unwrap();
        let (state, _) = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if row == 1 {
            return true;
        }
    }
    
    // Check left-right diagonal
    let mut col = -1;
    for row in -1..=1 {
        let pos = CellPosition { row, col };
        if pos == picked_pos {
            if row == 1 {
                return true;
            }
            col += 1;
            continue;
        }

        let ent = board.0.get(&pos).unwrap();
        let (state, _) = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if row == 1 {
            return true;
        }
        
        col += 1;
    }
    
    // Check right-left diagonal
    let mut col = 1;
    for row in -1..=1 {
        let pos = CellPosition { row, col };
        if pos == picked_pos {
            if row == 1 {
                return true;
            }
            col -= 1;
            continue;
        }

        let ent = board.0.get(&pos).unwrap();
        let (state, _) = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if row == 1 {
            return true;
        }
        
        col -= 1;
    }
    
    false
}

fn handle_game_over(game_state: &mut GameState) {
    let winner = match game_state {
        GameState::XTurn => Ok("X"),
        GameState::OTurn => Ok("O"),
        _ => Err("Game state is already GameOver")
    }.unwrap();

    println!("{winner} won!");
}