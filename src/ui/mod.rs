use std::time::Duration;

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
            .add_system(update_turn_text.in_schedule(OnEnter(GameState::OTurn)))
            .add_system(blinking);
    }
}

fn update_turn_text(
    mut commands: Commands,
    mut turn_text_qry: Query<(Entity, &mut Visibility, &TurnText)>,
    game_state: Res<State<GameState>>,
) {
    for (ent, mut vis, turn_text) in turn_text_qry.iter_mut() {
        match game_state.0 {
            GameState::XTurn => {
                if *turn_text == TurnText::X {
                    *vis = Visibility::Visible;
                    commands.entity(ent).insert(BlinkingTimer::new(
                        Duration::from_millis(200),
                        Duration::from_millis(50),
                    ));
                } else {
                    *vis = Visibility::Hidden;
                }
            },
            GameState::OTurn => {
                if *turn_text == TurnText::X {
                    *vis = Visibility::Hidden;
                } else {
                    *vis = Visibility::Visible;
                    commands.entity(ent).insert(BlinkingTimer::new(
                        Duration::from_millis(200),
                        Duration::from_millis(50),
                    ));
                }
            },
            GameState::GameOver => (),
        }
    }
}

fn show_game_over_popup(
    mut commands: Commands,
    mut game_over_evt_rdr: EventReader<GameOverEvent>,
    cell_qry: Query<(Entity, &CellState, &CellPosition)>,
    mat_handles: Res<MaterialHandles>,
) {
    for evt in game_over_evt_rdr.iter() {
        let ent = evt.last_picked_cell_ent;
        let state = evt.last_picked_cell_state;
    }
}

pub fn blinking(
    mut commands: Commands,
    mut flashing_qry: Query<(Entity, &mut BlinkingTimer, &mut Visibility)>,
    time: Res<Time>,
) {
    for (ent, mut timer, mut vis) in flashing_qry.iter_mut() {
        timer.tick(time.delta());
        if timer.just_blinked() {
            *vis = if *vis == Visibility::Visible {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        }
        if timer.just_finished() {
            *vis = Visibility::Visible;
            commands.entity(ent)
                .remove::<BlinkingTimer>();
        }
    }
}