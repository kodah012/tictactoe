use bevy::{prelude::*, window::PresentMode, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    DefaultPickingPlugins,
    PickableBundle,
    PickingCameraBundle, PickingEvent, Hover, Highlighting, NoDeselect, PickingPlugin, InteractablePickingPlugin, PickingPluginsState,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


#[derive(Resource)]
struct MaterialHandles {
    pub initial: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub clicked: Handle<ColorMaterial>,
}


fn main() {
    App::new()
        .insert_resource(PickingPluginsState {
            enable_picking: true,
            enable_highlighting: false,
            enable_interacting: true,
        })
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1600.,
                    height: 900.,
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
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_board)
        .add_system_to_stage(CoreStage::PostUpdate, handle_picking)
        .run();
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_size = 128.;

    let board_ent = commands.spawn(SpatialBundle::default())
        .insert(Name::new("Board"))
        .id();
    
    let initial_mat = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.),
        ..default()
    });

    let hovered_mat = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.5),
        ..default()
    });
    
    let clicked_mat = materials.add(ColorMaterial {
        color: Color::rgba(1., 0., 0., 0.5),
        ..default()
    });
    
    commands.insert_resource(MaterialHandles {
        initial: initial_mat.clone(),
        hovered: hovered_mat.clone(),
        clicked: clicked_mat.clone(),
    });
    
    for row in -1..=1 {
        for col in -1..=1 {
            let cell_ent = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::from_scale(Vec3::splat(128.))
                        .with_translation(
                            Vec3::new(col as f32 * tile_size, row as f32 * tile_size, 0.)
                        ),
                    material: initial_mat.clone(),
                    ..default()
                },
                PickableBundle::default(),
            )).id();
            commands.entity(board_ent).add_child(cell_ent);
        }
    }
}


// Goal now is when you click on a mesh, it changes color permanently.
// hovering should not change color. only clicking.
// Instead of using the built-in highlighting from bevy_mod_picking,
// change the materials manually using HoverEvents and PickingEvents

fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mat_handles: Res<MaterialHandles>,
) {
    events.iter().for_each(|event| {
        match event {
            PickingEvent::Clicked(ent) => {
                commands.entity(*ent).insert(mat_handles.clicked.clone());
            },
            _ => (),
        }
    });
}
