use bevy::{prelude::*, utils::HashMap, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle};
use crate::data::*;

mod gameover;
pub use gameover::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOverEvent>()
            .add_state::<GameState>()
            .add_system(update_game_state)
            .add_systems((
                highlight_winning_cells, show_game_over_popup
            ).in_schedule(OnEnter(GameState::GameOver)))
            .register_type::<CellState>()
            .register_type::<CellPosition>();
    }
}
