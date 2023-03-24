use bevy::{prelude::*, utils::HashMap, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, PickingEvent, HoverEvent};
use crate::resources::*;

mod data;
use data::*;

mod systems;
use systems::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOverEvent>()
            .add_state::<GameState>()
            .add_startup_system(spawn_board)
            .add_systems((handle_hover, handle_picking).in_set(OnUpdate(GameState::XTurn)))
            .add_systems((handle_hover, handle_picking).in_set(OnUpdate(GameState::OTurn)))
            .add_systems((
                highlight_winning_cells, show_game_over_popup
            ).in_schedule(OnEnter(GameState::GameOver)))
            .register_type::<CellState>()
            .register_type::<CellPosition>();
    }
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
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_over_evt_wtr: EventWriter<GameOverEvent>,
    game_state: ResMut<State<GameState>>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    cell_qry: Query<(&CellState, &CellPosition)>,
    board: Res<Board>,
) {
    events.iter().for_each(|event| {
        let curr_state = &game_state.0;
        if *curr_state == GameState::GameOver { return; }

        match event {
            PickingEvent::Clicked(ent) => {
                let (state, cell_pos) = cell_qry.get(*ent).unwrap();
                if *state == CellState::None {
                    let sprite_index = if *curr_state == GameState::XTurn { tex_atlas_indices.x_index } else { tex_atlas_indices.o_index };
                    let cell_state = if *curr_state == GameState::XTurn { CellState::X } else { CellState::O };
                    
                    let sprite_ent = commands.spawn(SpriteSheetBundle {
                        texture_atlas: tex_atlas_handle.0.clone_weak(),
                        sprite: TextureAtlasSprite::new(sprite_index),
                        transform: Transform::from_scale(Vec3::splat(0.05)),
                        ..default()
                    }).id();

                    commands.entity(*ent)
                        .insert(cell_state)
                        .add_child(sprite_ent);

                    let winning_positions = check_game_over(&cell_qry, (&cell_state, cell_pos), &board);
                    if let Some(positions) = winning_positions {
                        next_game_state.set(GameState::GameOver);
                        game_over_evt_wtr.send(GameOverEvent(positions));
                    } else {
                        let new_state = if *curr_state == GameState::XTurn { GameState::OTurn } else { GameState::XTurn };
                        next_game_state.set(new_state);
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
    game_state: ResMut<State<GameState>>,
) {
    events.iter().for_each(|event| {
        let curr_state = &game_state.0;
        if *curr_state == GameState::GameOver { return; }
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
