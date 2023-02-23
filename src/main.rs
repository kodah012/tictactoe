use bevy::{prelude::*, window::PresentMode, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{
    DebugEventsPickingPlugin,
    DefaultPickingPlugins,
    PickableBundle,
    PickingCameraBundle, PickingEvent,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 800.,
                    height: 600.,
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
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_board)
        //.add_startup_system(setup)
        //.add_system_to_stage(CoreStage::PostUpdate, print_events)
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
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    ..default()
                },
                PickableBundle::default(),
                Name::new("Cell")
            )).id();
            commands.entity(board_ent).add_child(cell_ent);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        PickableBundle::default(),
    ));
    commands.spawn((
        Camera2dBundle::default(),
        PickingCameraBundle::default(),
    ));
}

fn print_events(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event?! {:?}", e),
            PickingEvent::Clicked(e) => info!("Gee willikers, it's a click! {:?}", e),
        }
    }
}
