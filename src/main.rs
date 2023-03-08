use bevy::{prelude::*, window::PresentMode, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    DefaultPickingPlugins,
    PickableBundle,
    PickingCameraBundle, PickingEvent, Hover, Highlighting, NoDeselect, PickingPlugin, InteractablePickingPlugin, PickingPluginsState, HoverEvent, PickableMesh,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_loopless::prelude::IntoConditionalSystem;

#[derive(Resource, Clone, Copy)]
struct Params {
    tile_size: f32,
    window_width: f32,
    window_height: f32,
}

#[derive(Resource)]
struct MaterialHandles {
    pub initial: Handle<ColorMaterial>,
    pub initial_hovered: Handle<ColorMaterial>,
    pub picked: Handle<ColorMaterial>,
    pub picked_hovered: Handle<ColorMaterial>,
}

#[derive(Resource)]
struct TextureAtlasHandle(Handle<TextureAtlas>);

#[derive(Resource)]
struct TextureAtlasIndices {
    pub x_index: usize,
    pub o_index: usize,
}

#[derive(Component, Reflect, Eq, PartialEq)]
enum CellState {
    None,
    X,
    O,
}

fn main() {
    let params = Params {
        tile_size: 128.,
        window_width: 1600.,
        window_height: 900.,
    };

    App::new()
        .insert_resource(PickingPluginsState {
            enable_picking: true,
            enable_highlighting: false,
            enable_interacting: true,
        })
        .insert_resource(params)

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

        .run();
}


fn init_textures(
    mut commands: Commands,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let tex_handle = asset_server.load("../assets/atlas.png");
    let mut tex_atlas = TextureAtlas::new_empty(tex_handle, Vec2::new(248., 119.));

    let x_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 97.),
        max: Vec2::new(208., 113.),
    });
    let o_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(211., 97.),
        max: Vec2::new(227., 113.),
    });
    commands.insert_resource(TextureAtlasIndices {
        x_index,
        o_index,
    });

    let tex_atlas_handle = tex_atlases.add(tex_atlas);
    commands.insert_resource(TextureAtlasHandle(tex_atlas_handle));

    /*
    commands.spawn(
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(x_index),
            texture_atlas: tex_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.)),
            ..default()
        }
    );
    */
    
}

fn init_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let initial_mat = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.),
        ..default()
    });

    let initial_hovered_mat = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.3),
        ..default()
    });
    
    let clicked_mat = materials.add(ColorMaterial {
        color: Color::rgba(1., 0., 0., 0.5),
        ..default()
    });
    
    let clicked_hovered_mat = materials.add(ColorMaterial {
        color: Color::rgba(1., 0., 0., 0.8),
        ..default()
    });
    
    commands.insert_resource(MaterialHandles {
        initial: initial_mat,
        initial_hovered: initial_hovered_mat,
        picked: clicked_mat,
        picked_hovered: clicked_hovered_mat,
    });
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mat_handles: Res<MaterialHandles>,
    params: Res<Params>,
) {
    let board_ent = commands.spawn(SpatialBundle::default())
        .insert(Name::new("Board"))
        .id();
    
    for row in -1..=1 {
        for col in -1..=1 {
            let transform = Transform::from_scale(Vec3::splat(128.))
                .with_translation(
                    Vec3::new(col as f32 * params.tile_size, row as f32 * params.tile_size, 0.)
                );
            let cell_ent = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform,
                    material: mat_handles.initial.clone_weak(),
                    ..default()
                },
                PickableBundle::default(),
                CellState::None,
            )).id();
            commands.entity(board_ent).add_child(cell_ent);
        }
    }
}


fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mat_handles: Res<MaterialHandles>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    cell_q: Query<(&CellState, &Transform)>,
) {
    events.iter().for_each(|event| {
        match event {
            PickingEvent::Clicked(ent) => {
                let (state, transform) = cell_q.get(*ent).unwrap();
                if *state == CellState::None {
                    commands.entity(*ent)
                        .insert(mat_handles.picked.clone_weak())
                        .insert(CellState::X)
                        .insert(SpriteSheetBundle {
                            texture_atlas: tex_atlas_handle.0.clone(),
                            transform: Transform::from_translation(transform.translation)
                                .with_scale(Vec3::splat(3.)),
                            ..default()
                        });
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
    state_q: Query<&CellState>,
) {
    events.iter().for_each(|event| {
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(ent)) => {
                let state = state_q.get(*ent).unwrap();
                match *state {
                    CellState::None => commands.entity(*ent).insert(mat_handles.initial_hovered.clone_weak()),
                    _ => commands.entity(*ent).insert(mat_handles.picked_hovered.clone_weak()),
                };
            },
            PickingEvent::Hover(HoverEvent::JustLeft(ent)) => {
                let state = state_q.get(*ent).unwrap();
                match *state {
                    CellState::None => commands.entity(*ent).insert(mat_handles.initial.clone_weak()),
                    _ => commands.entity(*ent).insert(mat_handles.picked.clone_weak()),
                };
            },
            _ => (),
        }
    });
}
