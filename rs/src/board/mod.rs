use std::usize;

use wasm_bindgen::prelude::*;

use crate::rules::{self, Movement};

#[wasm_bindgen]
pub struct Board {
    squares: [[Option<Piece>; 4]; 8],
}

// #[wasm_bindgen]
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
        if row >= 8 || row >= 8 {
            return None;
        }
        match (row % 2, col % 2) {
            (0, 0) => {
                let inner_col = col / 2;
                match &self.squares[row][inner_col] {
                    Some(piece) => Some(GamePiece {
                        color: piece.color,
                        is_king: piece.is_king,
                        row,
                        col,
                    }),
                    _ => None,
                }
            }
            (1, 1) => {
                let inner_col = (col - 1) / 2;
                match &self.squares[row][inner_col] {
                    Some(piece) => Some(GamePiece {
                        color: piece.color,
                        is_king: piece.is_king,
                        row,
                        col,
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn all_pieces(&self) -> Vec<GamePiece> {
        self.get_normalized_pieces().collect()
    }

    pub fn pieces(&self, color: Color) -> Vec<GamePiece> {
        self.get_normalized_pieces()
            .filter(move |piece| piece.color == color)
            .collect()
    }

    pub fn moves_for(&self, row: usize, col: usize) -> Vec<Movement> {
        rules::get_moves(&self, row, col)
    }

    pub fn get_movable_pieces(&self, color: Color) -> Vec<GamePiece> {
        self.get_normalized_pieces()
            .filter(move |piece| piece.color == color)
            .filter(|piece| !self.moves_for(piece.row, piece.col).is_empty())
            .collect()
    }
}

// Not exported
impl Board {
    pub(crate) fn from(squares: [[Option<Piece>; 4]; 8]) -> Board {
        Board { squares }
    }

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
                            col: col * 2 + row % 2,
                        }
                    })
            })
            .flatten()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: Color,
    pub is_king: bool,
    pub row: usize,
    pub col: usize,
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

#[cfg(test)]
mod tests;
