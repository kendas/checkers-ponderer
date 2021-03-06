use std::convert::TryFrom;
use std::usize;

use wasm_bindgen::prelude::*;

use crate::rules::{self, Movement, MovementType};

#[derive(Clone, Copy)]
pub struct Board {
    pub(crate) squares: [[Option<Piece>; 4]; 8],
}

impl Board {
    pub fn new() -> Board {
        let black_piece = Some(Piece {
            color: Color::Black,
            is_king: false,
        });
        let white_piece = Some(Piece {
            color: Color::White,
            is_king: false,
        });
        Board {
            squares: [
                [black_piece; 4],
                [black_piece; 4],
                [black_piece; 4],
                [None; 4],
                [None; 4],
                [white_piece; 4],
                [white_piece; 4],
                [white_piece; 4],
            ],
        }
    }

    pub fn count_pieces(&self, color: Color) -> usize {
        self.squares
            .iter()
            .flatten()
            .filter(|p| p.map_or(false, |p| p.color == color))
            .count()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<GamePiece> {
        match get_internal_col(row, col) {
            Some(inner_col) => match &self.squares[row][inner_col] {
                Some(piece) => Some(GamePiece {
                    color: piece.color,
                    is_king: piece.is_king,
                    row,
                    col,
                }),
                _ => None,
            },
            None => None,
        }
    }

    pub fn all_pieces(&self) -> impl Iterator<Item = GamePiece> + '_ {
        self.get_normalized_pieces()
    }

    pub fn pieces(&self, color: Color) -> impl Iterator<Item = GamePiece> + '_ {
        self.get_normalized_pieces()
            .filter(move |piece| piece.color == color)
    }

    pub fn moves_for(&self, row: usize, col: usize) -> Vec<Movement> {
        rules::get_moves(&self, row, col)
    }

    pub fn get_movable_pieces(&self, color: Color) -> impl Iterator<Item = GamePiece> + '_ {
        let (forced, free): (Vec<_>, Vec<_>) = self
            .get_normalized_pieces()
            .filter(move |piece| piece.color == color)
            .map(|piece| (piece, rules::get_moves(&self, piece.row, piece.col)))
            .filter(|(_, movements)| !movements.is_empty())
            .partition(|(_, m)| rules::has_forced_moves(m));
        if !forced.is_empty() { forced } else { free }
            .into_iter()
            .map(|(p, _)| p)
    }

    pub fn make_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> Result<Board, InvalidMove> {
        let valid_move = rules::get_moves(&self, from_row, from_col)
            .into_iter()
            .find(|m| m.row == to_row && m.col == to_col);
        match valid_move {
            Some(move_) => {
                let mut squares = self.squares.clone();
                match move_.movement_type {
                    MovementType::Forced => {
                        let row = (from_row + to_row) / 2;
                        let col =
                            get_internal_col(row, (from_col + to_col) / 2).unwrap();
                        squares[row][col] = None;
                    }
                    _ => {}
                }
                let col = get_internal_col(from_row, from_col).unwrap();
                let mut piece = squares[from_row][col].take().unwrap();
                let col = get_internal_col(to_row, to_col).unwrap();
                match piece.color {
                    Color::White => piece.is_king |= to_row == 0,
                    Color::Black => piece.is_king |= to_row == 7,
                }
                squares[to_row][col] = Some(piece);
                Ok(Board { squares })
            }
            None => Err(InvalidMove),
        }
    }
}

// Not exported
impl Board {
    fn get_normalized_pieces(&self) -> impl Iterator<Item = GamePiece> + '_ {
        self.squares
            .iter()
            .enumerate()
            .map(|(row, pieces)| {
                pieces
                    .iter()
                    .enumerate()
                    .filter(|(_, piece)| piece.is_some())
                    .map(move |(col, piece)| {
                        let piece = piece.as_ref().unwrap();
                        GamePiece {
                            color: piece.color,
                            is_king: piece.is_king,
                            row,
                            col: get_external_col(row, col),
                        }
                    })
            })
            .flatten()
    }
}

fn get_internal_col(row: usize, col: usize) -> Option<usize> {
    if row >= 8 || row >= 8 {
        return None;
    }
    match (row % 2, col % 2) {
        (1, 0) => Some(col / 2),
        (0, 1) => Some((col - 1) / 2),
        _ => None,
    }
}

fn get_external_col(row: usize, col: usize) -> usize {
    col * 2 + (row + 1) % 2
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: Color,
    pub is_king: bool,
    pub row: usize,
    pub col: usize,
}

impl GamePiece {
    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.into()
    }
}

impl Into<Vec<u8>> for GamePiece {
    fn into(self) -> Vec<u8> {
        vec![
            self.color as u8,
            self.is_king as u8,
            u8::try_from(self.row).unwrap(),
            u8::try_from(self.col).unwrap(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub is_king: bool,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub struct InvalidMove;

#[cfg(test)]
mod tests;
