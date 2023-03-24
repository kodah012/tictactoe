use bevy::prelude::*;

use crate::{
    resources::MaterialHandles,
    logic::data::*
};

pub fn highlight_winning_cells(
    mut commands: Commands,
    mut game_over_evt_rdr: EventReader<GameOverEvent>,
    cell_qry: Query<(Entity, &CellState, &CellPosition)>,
    mat_handles: Res<MaterialHandles>,
) {
    for evt in game_over_evt_rdr.iter() {
        let winning_positions = evt.0;
        for (ent, state, pos) in cell_qry.iter() {
            if winning_positions.contains(pos) {
                commands.entity(ent).insert(mat_handles.winner.clone_weak());
            }
        }
    }
}

pub fn show_game_over_popup(
    mut commands: Commands,
    mut game_over_evt_rdr: EventReader<GameOverEvent>,
    cell_qry: Query<(Entity, &CellState, &CellPosition)>,
    mat_handles: Res<MaterialHandles>,
) {
}

pub fn check_game_over(
    cell_qry: &Query<(&CellState, &CellPosition)>,
    picked_cell: (&CellState, &CellPosition),
    board: &Board,
) -> Option<[CellPosition; 3]> {
    let picked_state = *picked_cell.0;
    let picked_pos = *picked_cell.1;

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
        let (state, _) = cell_qry.get(*ent).unwrap();
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
        let (state, _) = cell_qry.get(*ent).unwrap();
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
        let (state, _) = cell_qry.get(*ent).unwrap();
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
        let (state, _) = cell_qry.get(*ent).unwrap();
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