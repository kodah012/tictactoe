use bevy::{prelude::*, window::{PresentMode, WindowResolution}};
use bevy_mod_picking::{
    DefaultPickingPlugins,
    PickingCameraBundle, PickingPluginsState,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod data;
use data::*;

mod input;
use input::InputPlugin;

mod logic;
use logic::LogicPlugin;

mod ui;
use ui::UiPlugin;

fn main() {
    let params = Params {
        tile_size: 128.,
        window_width: 506.25,
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
        .add_plugin(InputPlugin)
        .add_plugin(UiPlugin)
        
        //.add_plugin(WorldInspectorPlugin::new())
        .register_type::<TextureAtlasSprite>()

        .run();
}


