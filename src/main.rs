use bevy::{prelude::*, window::{PresentMode, WindowResolution}};
use bevy_mod_picking::{
    DefaultPickingPlugins,
    PickingCameraBundle, PickingPluginsState,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod logic;
use logic::LogicPlugin;

mod resources;
use resources::*;


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
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    resolution: WindowResolution::new(params.window_width, params.window_height),
                    title: "Tic-Tac-Toe".to_string(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(LogicPlugin)
        
        .add_startup_system(init_materials.in_base_set(StartupSet::PreStartup))
        .add_startup_system(init_textures.in_base_set(StartupSet::PreStartup))
        .add_startup_system(spawn_camera)

        .add_plugin(WorldInspectorPlugin::new())
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
