use std::iter::empty;

use bevy::{prelude::*, window::PresentMode, sprite::MaterialMesh2dBundle, utils::HashMap};
use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    DefaultPickingPlugins,
    PickableBundle,
    PickingCameraBundle, PickingEvent, Hover, Highlighting, NoDeselect, PickingPlugin, InteractablePickingPlugin, PickingPluginsState, HoverEvent, PickableMesh,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource, Clone, Copy)]
struct Params {
    tile_size: f32,
    window_width: f32,
    window_height: f32,
}

#[derive(Resource)]
struct MaterialHandles {
    pub transparent: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct TextureAtlasHandle(Handle<TextureAtlas>);

#[derive(Resource)]
struct TextureAtlasIndices {
    pub bg_index: usize,
    pub x_index: usize,
    pub o_index: usize,
}

#[derive(Resource)]
struct Board(HashMap<CellPosition, Entity>);

#[derive(Resource, Eq, PartialEq)]
enum GameState {
    XTurn,
    OTurn,
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


fn main() {
    let params = Params {
        tile_size: 128.,
        window_width: 1600.,
        window_height: 900.,
    };

    App::new()
        .add_startup_system_to_stage(StartupStage::PreStartup, move |mut commands: Commands| {
            commands.insert_resource(PickingPluginsState {
                enable_picking: true,
                enable_highlighting: false,
                enable_interacting: true,
            });
            commands.insert_resource(params);
            commands.insert_resource(GameState::XTurn);
        })

        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: params.window_width,
                    height: params.window_height,
                    title: "Tic-Tac-Toe".to_string(),
                    present_mode: PresentMode::Fifo,
                    resizable: false,
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(DefaultPickingPlugins)
        //.add_plugin(DebugEventsPickingPlugin)
        
        .add_startup_system_to_stage(StartupStage::PreStartup, init_materials)
        .add_startup_system_to_stage(StartupStage::PreStartup, init_textures)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_camera)
        .add_system_to_stage(CoreStage::PostUpdate, handle_hover)
        .add_system_to_stage(CoreStage::PostUpdate, handle_picking)

        .add_plugin(WorldInspectorPlugin)
        .register_type::<CellState>()
        .register_type::<TextureAtlasSprite>()
        .register_type::<CellPosition>()

        .run();
}


fn init_textures(
    mut commands: Commands,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let tex_handle = asset_server.load("../assets/atlas.png");
    let mut tex_atlas = TextureAtlas::new_empty(tex_handle, Vec2::new(248., 119.));

    let bg_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(125., 3.),
        max: Vec2::new(189., 116.),
    });
    let x_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 97.),
        max: Vec2::new(208., 113.),
    });
    let o_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(211., 97.),
        max: Vec2::new(227., 113.),
    });
    commands.insert_resource(TextureAtlasIndices {
        bg_index,
        x_index,
        o_index,
    });

    let tex_atlas_handle = tex_atlases.add(tex_atlas);
    commands.insert_resource(TextureAtlasHandle(tex_atlas_handle));
}

fn init_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transparent = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.),
        ..default()
    });
    
    let hovered = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.3),
        ..default()
    });
    
    commands.insert_resource(MaterialHandles {
        transparent,
        hovered,
    });
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        ..default()
    })
        .insert(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
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
            let transform = Transform::from_scale(Vec3::splat(params.tile_size * 1.1))
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
    mat_handles: Res<MaterialHandles>,
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

                    let new_state = if *game_state == GameState::XTurn { GameState::OTurn } else { GameState::XTurn };
                    *game_state = new_state;
                    
                    let game_over = check_game_over(&cell_qry, (&cell_state, cell_pos), &board);
                    println!("{}", game_over);
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
