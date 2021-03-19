use wasm_bindgen::prelude::*;

use crate::board::{Board, Color, Piece};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Movement {
    pub movement_type: MovementType,
    pub row: usize,
    pub col: usize,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MovementType {
    Free,
    Forced,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Increase,
    Decrease,
}

macro_rules! matches(
    ($e:expr, $p:pat) => (
        match $e {
            $p => true,
            _ => false
        }
    )
);

pub fn get_moves(board: &Board, row: usize, col: usize) -> Vec<Movement> {
    match board.get(row, col) {
        Some(piece) => {
            let possibilities = get_possibilities(&piece);

            let mut moves = vec![];
            for (row_direction, col_direction) in possibilities {
                if let Some((row, col)) = get_next(row, col, row_direction, col_direction) {
                    match board.get(row, col) {
                        None => moves.push(Movement {
                            movement_type: MovementType::Free,
                            row,
                            col,
                        }),
                        Some(other_piece) => {
                            if other_piece.color != piece.color {
                                if let Some((row, col)) =
                                    get_next(row, col, row_direction, col_direction)
                                {
                                    if board.get(row, col).is_none() {
                                        moves.push(Movement {
                                            movement_type: MovementType::Forced,
                                            row,
                                            col,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if moves
                .iter()
                .any(|m| matches!(m.movement_type, MovementType::Forced))
            {
                moves
                    .into_iter()
                    .filter(|m| matches!(m.movement_type, MovementType::Forced))
                    .collect()
            } else {
                moves
            }
        }
        None => Default::default(),
    }
}

fn get_next(
    row: usize,
    col: usize,
    row_direction: Direction,
    col_direction: Direction,
) -> Option<(usize, usize)> {
    use Direction::*;
    if matches!(row_direction, Decrease) && row <= 0 {
        None
    } else if matches!(row_direction, Increase) && row >= 7 {
        None
    } else if matches!(col_direction, Decrease) && col <= 0 {
        None
    } else if matches!(col_direction, Increase) && col >= 7 {
        None
    } else {
        let row = match row_direction {
            Increase => row + 1,
            Decrease => row - 1,
        };
        let col = match col_direction {
            Increase => col + 1,
            Decrease => col - 1,
        };
        Some((row, col))
    }
}

fn get_possibilities(piece: &Piece) -> Vec<(Direction, Direction)> {
    use Direction::*;
    let direction = match piece.color {
        Color::White => Decrease,
        Color::Black => Increase,
    };
    let mut possibilities = vec![(direction, Decrease), (direction, Increase)];
    if piece.is_king {
        let direction = match direction {
            Increase => Decrease,
            Decrease => Increase,
        };
        possibilities.push((direction, Decrease));
        possibilities.push((direction, Increase));
    }
    possibilities
}

#[cfg(test)]
mod tests;
