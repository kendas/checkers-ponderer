use crate::rules::{self, Movement};

pub struct Board {
    // TODO: Remove pub
    pub squares: [[Option<Piece>; 4]; 8],
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

    pub fn piece_count(&self, color: Color) -> usize {
        self.squares
            .iter()
            .flatten()
            .filter(|p| p.map_or(false, |p| p.color == color))
            .count()
    }

    pub fn get(&self, position: Position) -> Option<Piece> {
        if position.0 >= 8 || position.1 >= 8 {
            return None;
        }
        match position {
            Position(row, col) if row % 2 == 0 => {
                if col % 2 == 1 {
                    None
                } else {
                    let col = col / 2;
                    match self.squares[row][col] {
                        Some(piece) => Some(piece),
                        None => None,
                    }
                }
            }
            Position(row, col) => {
                if col % 2 == 0 {
                    None
                } else {
                    let col = (col - 1) / 2;
                    match self.squares[row][col] {
                        Some(piece) => Some(piece),
                        None => None,
                    }
                }
            }
        }
    }

    pub fn all_pieces(&self) -> impl Iterator<Item = (&Piece, Position)> {
        self.squares
            .iter()
            .enumerate()
            .map(|(row, pieces)| {
                pieces
                    .iter()
                    .enumerate()
                    .filter(|(_, piece)| piece.is_some())
                    .map(move |(col, piece)| (piece.as_ref().unwrap(), Position(row, col * 2 + row % 2)))
            })
            .flatten()
    }

    pub fn pieces(&self, color: Color) -> impl Iterator<Item = (&Piece, Position)> {
        self.all_pieces().filter(move |(piece, _)| piece.color == color)
    }

    pub fn moves_for(&self, position: Position) -> Vec<Movement> {
        rules::get_moves(self, position)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub is_king: bool,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub usize, pub usize);

#[cfg(test)]
mod tests;
