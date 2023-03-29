use super::*;
pub struct GameOverEvent {
    pub picked_cell_ent: Entity,
    pub picked_cell_state: CellState,
    pub winning_positions: [CellPosition; 3]
}
pub struct CellPickedEvent {
    pub entity: Entity,
    pub state: CellState,
    pub position: CellPosition,
}
