use wasm_bindgen::prelude::*;

use crate::board::{Board, Color, GamePiece};

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
            let moves = get_moves_from_possibilities(&board, &piece, possibilities);

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
) -> Result<(usize, usize), ()> {
    use Direction::*;
    if matches!(row_direction, Decrease) && row <= 0 {
        Err(())
    } else if matches!(row_direction, Increase) && row >= 7 {
        Err(())
    } else if matches!(col_direction, Decrease) && col <= 0 {
        Err(())
    } else if matches!(col_direction, Increase) && col >= 7 {
        Err(())
    } else {
        let row = match row_direction {
            Increase => row + 1,
            Decrease => row - 1,
        };
        let col = match col_direction {
            Increase => col + 1,
            Decrease => col - 1,
        };
        Ok((row, col))
    }
}

fn get_possibilities(piece: &GamePiece) -> Vec<(Direction, Direction)> {
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

fn get_moves_from_possibilities(
    board: &Board,
    piece: &GamePiece,
    possibilities: Vec<(Direction, Direction)>,
) -> Vec<Movement> {
    let mut moves = vec![];
    for (row_direction, col_direction) in possibilities {
        if let Ok((row, col)) = get_next(piece.row, piece.col, row_direction, col_direction) {
            match board.get(row, col) {
                None => moves.push(Movement {
                    movement_type: MovementType::Free,
                    row,
                    col,
                }),
                Some(other_piece) => {
                    if other_piece.color != piece.color {
                        if let Ok((row, col)) = get_next(row, col, row_direction, col_direction) {
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
    moves
}

#[cfg(test)]
mod tests;
