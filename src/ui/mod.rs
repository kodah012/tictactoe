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
            .add_system(show_game_over_popup.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(update_blinking_timers)
            .add_system(update_delay_timers);
    }
}

fn update_turn_text(
    mut commands: Commands,
    mut turn_text_qry: Query<(Entity, &mut TextureAtlasSprite), With<TurnText>>,
    game_state: Res<State<GameState>>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
) {
    for (ent, mut sprite) in turn_text_qry.iter_mut() {
        match game_state.0 {
            GameState::XTurn => {
                *sprite = TextureAtlasSprite::new(tex_atlas_indices.x_turn);
                commands.entity(ent).insert(BlinkingTimer::new(
                    Duration::from_millis(200),
                    Duration::from_millis(50),
                ));
            },
            GameState::OTurn => {
                *sprite = TextureAtlasSprite::new(tex_atlas_indices.o_turn);
                commands.entity(ent).insert(BlinkingTimer::new(
                    Duration::from_millis(200),
                    Duration::from_millis(50),
                ));
            },
            GameState::GameOver => (),
        }
    }
}

fn show_game_over_popup(
    mut commands: Commands,
    mut game_over_evt_rdr: EventReader<GameOverEvent>,
    mut popup_qry: Query<(Entity, &mut Visibility), With<GameOverPopup>>,
) {
    for evt in game_over_evt_rdr.iter() {
        let cell_ent = evt.last_picked_cell_ent;
        let state = evt.last_picked_cell_state;
        let (popup_ent, mut vis) = popup_qry.single_mut();
        commands.entity(popup_ent).insert(DelayTimer(
            Timer::new(Duration::from_millis(1000), TimerMode::Once)
        ));
    }
}

fn update_blinking_timers(
    mut commands: Commands,
    mut blinking_qry: Query<(Entity, &mut BlinkingTimer, &mut Visibility)>,
    time: Res<Time>,
) {
    for (ent, mut timer, mut vis) in blinking_qry.iter_mut() {
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

fn update_delay_timers(
    mut commands: Commands,
    mut timer_qry: Query<(Entity, &mut DelayTimer, &mut Visibility)>,
    time: Res<Time>,
) {
    for (ent, mut timer, mut vis) in timer_qry.iter_mut() {
        timer.0.tick(time.delta());
        *vis = if timer.0.just_finished() {
            commands.entity(ent).remove::<DelayTimer>();
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}