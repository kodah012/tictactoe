use bevy::{prelude::*, window::PresentMode, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    DefaultPickingPlugins,
    PickableBundle,
    PickingCameraBundle, PickingEvent, Hover, Highlighting,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
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
        //.add_plugin(DebugEventsPickingPlugin)
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
    
    for row in -1..=1 {
        for col in -1..=1 {
            let cell_ent = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::from_scale(Vec3::splat(128.))
                        .with_translation(
                            Vec3::new(col as f32 * tile_size, row as f32 * tile_size, 0.)
                        ),
                    material: materials.add(ColorMaterial {
                        color: Color::rgba(0., 0., 0., 0.),
                        ..default()
                    }),
                    ..default()
                },
                PickableBundle::default(),
                Highlighting {
                    initial: materials.add(ColorMaterial {
                        color: Color::rgba(0., 0., 0., 0.),
                        ..default()
                    }),
                    hovered: None,
                    pressed: None,
                    selected: None,
                },
                Name::new("Cell")
            )).id();
            commands.entity(board_ent).add_child(cell_ent);
        }
    }
}


// Goal now is when you click on a mesh, it changes color permanently.
// hovering should not change color. only clicking.

fn handle_picking(
    mut events: EventReader<PickingEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    material_q: Query<&Handle<ColorMaterial>>,
) {
    events.iter().for_each(|event| {
        match event {
            PickingEvent::Clicked(ent) => {
                let mat = materials.get_mut(
                    material_q.get(*ent).unwrap()
                ).unwrap();
                mat.color = Color::RED;
            },
            _ => (),
        }
    });
}
