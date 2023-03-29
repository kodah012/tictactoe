use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, HoverEvent};
use crate::data::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<CellPickedEvent>()
            .add_systems((handle_cell_hover, handle_cell_picking).in_set(OnUpdate(GameState::XTurn)))
            .add_systems((handle_cell_hover, handle_cell_picking).in_set(OnUpdate(GameState::OTurn)));
    }
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
                let (state, cell_pos) = cell_qry.get(*ent).unwrap();
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
) {
    events.iter().for_each(|event| {
        let curr_state = &game_state.0;
        if *curr_state == GameState::GameOver { return; }
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(ent)) => {
                commands.entity(*ent).insert(mat_handles.hovered.clone_weak());
            },
            PickingEvent::Hover(HoverEvent::JustLeft(ent)) => {
                commands.entity(*ent).insert(mat_handles.transparent.clone_weak());
            },
            _ => (),
        }
    });
}
