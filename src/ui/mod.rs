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
            .add_startup_system(spawn_game_over_popup)
            .add_system(update_turn_text.in_schedule(OnEnter(GameState::XTurn)))
            .add_system(update_turn_text.in_schedule(OnEnter(GameState::OTurn)));
    }
}

fn update_turn_text(
    mut turn_text_qry: Query<(&mut Visibility, &TurnText)>,
    game_state: Res<State<GameState>>,
) {
    for (mut vis, turn_text) in turn_text_qry.iter_mut() {
        match game_state.0 {
            GameState::XTurn => {
                if *turn_text == TurnText::X {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            },
            GameState::OTurn => {
                if *turn_text == TurnText::X {
                    *vis = Visibility::Hidden;
                } else {
                    *vis = Visibility::Visible;
                }
            },
            GameState::GameOver => (),
        }
    }
}