use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};
use crate::data::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<CellPickedEvent>()
            .add_event::<PlayBtnClickedEvt>()
            .add_event::<QuitBtnClickedEvt>()
            .add_systems((handle_cell_hover, handle_cell_picking).in_set(OnUpdate(GameState::XTurn)))
            .add_systems((handle_cell_hover, handle_cell_picking).in_set(OnUpdate(GameState::OTurn)));
            //.add_system(handle_play_btn_clicked);
    }
}

fn handle_play_btn_clicked(
    mut picking_evt_rdr: EventReader<PickingEvent>,
    mut play_btn_evt_wtr: EventWriter<PlayBtnClickedEvt>,
    game_over_popup_qry: Query<(Entity, With<GameOverPopup>)>,
    play_btn_qry: Query<&PlayBtn>,
    name_qry: Query<&Name>,
) {
    picking_evt_rdr.iter().for_each(|evt| {
        match evt {
            PickingEvent::Clicked(ety) => {
                // FIXME: no entities detected with the PlayBtn component
                // maybe because it's a child of an the game over popup entity?
                if let Ok(name) = name_qry.get(*ety) {
                    println!("{}", name);
                }
                
                /*
                if let Ok(_) = play_btn_qry.get(*ent) {
                    play_btn_evt_wtr.send(PlayBtnClickedEvt);
                }
                */
            }
            _ => ()
        }
    });
}

fn handle_cell_picking(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mut cell_picked_evt_wtr: EventWriter<CellPickedEvent>,
    game_state: ResMut<State<GameState>>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    cell_qry: Query<(&CellState, &CellPosition)>,
) {
    events.iter().for_each(|event| {
        let curr_state = &game_state.0;
        if *curr_state == GameState::GameOver { return; }

        match event {
            PickingEvent::Clicked(ent) => {
                if let Ok((state, cell_pos)) = cell_qry.get(*ent) {
                    if *state == CellState::None {
                        let sprite_index = if *curr_state == GameState::XTurn {
                            tex_atlas_indices.x
                        } else {
                            tex_atlas_indices.o
                        };
                        let new_state = if *curr_state == GameState::XTurn { CellState::X } else { CellState::O };
                        
                        let sprite_ent = commands.spawn(SpriteSheetBundle {
                            texture_atlas: tex_atlas_handle.0.clone_weak(),
                            sprite: TextureAtlasSprite::new(sprite_index),
                            transform: Transform::from_scale(Vec3::splat(0.05)),
                            ..default()
                        }).id();

                        commands.entity(*ent)
                            .insert(new_state)
                            .add_child(sprite_ent);

                        cell_picked_evt_wtr.send(CellPickedEvent {
                            entity: *ent,
                            state: new_state,
                            position: *cell_pos,
                        });
                    }
                }
            },
            _ => (),
        }
    });
}

fn handle_cell_hover(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    mat_handles: Res<MaterialHandles>,
    game_state: ResMut<State<GameState>>,
    cell_state_qry: Query<&CellState>,
) {
    events.iter().for_each(|event| {
        let curr_state = &game_state.0;
        if *curr_state == GameState::GameOver { return; }
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(ent)) => {
                if let Ok(_) = cell_state_qry.get(*ent) {
                    commands.entity(*ent).insert(mat_handles.hovered.clone_weak());
                }
            },
            PickingEvent::Hover(HoverEvent::JustLeft(ent)) => {
                if let Ok(_) = cell_state_qry.get(*ent) {
                    commands.entity(*ent).insert(mat_handles.transparent.clone_weak());
                }
            },
            _ => (),
        }
    });
}
