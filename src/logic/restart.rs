use bevy::prelude::*;

use crate::data::PlayBtnClickedEvt;

pub fn restart_game(
    mut play_btn_evt_rdr: EventReader<PlayBtnClickedEvt>
) {
    for evt in play_btn_evt_rdr.iter() {
        println!("play button clicked");
    }
}