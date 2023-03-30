use std::time::Duration;

use bevy::prelude::*;

use crate::{
    data::MaterialHandles,
    logic::*,
};

pub fn highlight_winning_cells(
    mut commands: Commands,
    mut game_over_evt_rdr: EventReader<GameOverEvent>,
    cell_qry: Query<(Entity, &CellState, &CellPosition)>,
    mat_handles: Res<MaterialHandles>,
) {
    for evt in game_over_evt_rdr.iter() {
        let winning_positions = evt.winning_positions;
        for (ent, state, pos) in cell_qry.iter() {
            if winning_positions.contains(pos) {
                commands.entity(ent)
                    .insert(mat_handles.winner.clone_weak())
                    .insert(BlinkingTimer::new(
                        Duration::from_millis(500),
                        Duration::from_millis(50),
                    ));
            }
        }
    }
}


pub fn update_game_state(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_over_evt_wtr: EventWriter<GameOverEvent>,
    mut cell_picked_evt_rdr: EventReader<CellPickedEvent>,
    curr_game_state: Res<State<GameState>>,
    cell_qry: Query<&CellState>,
    board: Res<Board>,
) {
    for evt in cell_picked_evt_rdr.iter() {
        let ent = evt.entity;
        let state = evt.state;
        let pos = evt.position;
        let winning_positions = get_winning_positions(
            &cell_qry,
            (state, pos),
            &board
        );
        if let Some(positions) = winning_positions {
            next_game_state.set(GameState::GameOver);
            game_over_evt_wtr.send(GameOverEvent {
                last_picked_cell_ent: ent,
                last_picked_cell_state: state,
                winning_positions: positions,
            });
        } else {
            let new_state = if curr_game_state.0 == GameState::XTurn {
                GameState::OTurn
            } else {
                GameState::XTurn
            };
            next_game_state.set(new_state);
        }
    }
}

fn get_winning_positions(
    cell_qry: &Query<&CellState>,
    picked_cell: (CellState, CellPosition),
    board: &Board,
) -> Option<[CellPosition; 3]> {
    let picked_state = picked_cell.0;
    let picked_pos = picked_cell.1;

    // Check horizontal
    let mut game_over = false;
    for col in -1..=1 {
        if col == picked_pos.col {
            if col == 1 {
                game_over = true;
                break;
            }
            continue;
        }

        let pos = CellPosition { row: picked_pos.row, col };
        let ent = board.0.get(&pos).unwrap();
        let state = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if col == 1 {
            game_over = true;
            break;
        }
    }
    if game_over {
        let row = picked_pos.row;
        return Some([
            CellPosition { row, col: -1 },
            CellPosition { row, col: 0 },
            CellPosition { row, col: 1 },
        ]);
    }
    
    // Check vertical
    game_over = false;
    for row in -1..=1 {
        if row == picked_pos.row {
            if row == 1 {
                game_over = true;
                break;
            }
            continue;
        }

        let pos = CellPosition { row, col: picked_pos.col };
        let ent = board.0.get(&pos).unwrap();
        let state = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if row == 1 {
            game_over = true;
            break;
        }
    }
    if game_over {
        let col = picked_pos.col;
        return Some([
            CellPosition { row: -1, col },
            CellPosition { row: 0, col },
            CellPosition { row: 1, col },
        ]);
    }
    
    // Check left-right diagonal
    game_over = false;
    let mut col = -1;
    for row in -1..=1 {
        let pos = CellPosition { row, col };
        if pos == picked_pos {
            if row == 1 {
                game_over = true;
                break;
            }
            col += 1;
            continue;
        }

        let ent = board.0.get(&pos).unwrap();
        let state = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if row == 1 {
            game_over = true;
            break;
        }
        
        col += 1;
    }
    if game_over {
        return Some([
            CellPosition { row: -1, col: -1 },
            CellPosition { row: 0, col: 0 },
            CellPosition { row: 1, col: 1 },
        ]);
    }
    
    // Check right-left diagonal
    game_over = false;
    let mut col = 1;
    for row in -1..=1 {
        let pos = CellPosition { row, col };
        if pos == picked_pos {
            if row == 1 {
                game_over = true;
                break;
            }
            col -= 1;
            continue;
        }

        let ent = board.0.get(&pos).unwrap();
        let state = cell_qry.get(*ent).unwrap();
        if *state != picked_state {
            break;
        }
        if row == 1 {
            game_over = true;
            break;
        }
        
        col -= 1;
    }
    if game_over {
        return Some([
            CellPosition { row: -1, col: 1 },
            CellPosition { row: 0, col: 0 },
            CellPosition { row: 1, col: -1 },
        ]);
    }
    
    None
}