use bevy::{prelude::*, utils::HashMap, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};

use crate::data::*;

mod init;
use init::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_materials.in_base_set(StartupSet::PreStartup))
            .add_startup_system(init_textures.in_base_set(StartupSet::PreStartup))
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_board)
            .add_startup_system(spawn_turn_text)
            .add_startup_system(spawn_game_over_popup);
    }
}
