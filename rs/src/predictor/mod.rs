use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use crate::{
    board::{Board, Color},
    rules::MovementType,
};

pub struct Predictor {
    depth: u8,
    color: Color,
    cache: PredictionCache,
}

impl Predictor {
    pub fn new(board: Board, depth: u8, color: Color) -> Predictor {
        Predictor {
            depth,
            color,
            cache: PredictionCache::new(board),
        }
    }

    pub fn get_next_move(&mut self) -> Result<Move, NoMoreMoves> {
        self.cache.calculate_moves(2 * self.depth - 1, self.color);
        let result = self
            .cache
            .branches
            .iter()
            .max_by(|(_, a), (_, b)| a.get_score(self.color).cmp(&b.get_score(self.color)));
        match result {
            Some((move_, _)) => Ok(move_.clone()),
            None => Err(NoMoreMoves),
        }
    }

    pub fn register_move(&mut self, own_move: Move, oponent_move: Move) -> Result<(), InvalidMove> {
        if let Some(mut cache) = self.cache.branches.remove(&own_move) {
            if let Some(cache) = cache.branches.remove(&oponent_move) {
                self.cache = cache;
                return Ok(());
            }
        }
        Err(InvalidMove)
    }
}

struct PredictionCache {
    board: Board,
    branches: HashMap<Move, PredictionCache>,
}

impl PredictionCache {
    fn new(board: Board) -> PredictionCache {
        PredictionCache {
            board,
            branches: Default::default(),
        }
    }

    fn calculate_moves(&mut self, depth: u8, color: Color) {
        for piece in self.board.get_movable_pieces(color) {
            for movement in self.board.moves_for(piece.row, piece.col) {
                let move_ = Move {
                    from: Position {
                        row: piece.row,
                        col: piece.col,
                    },
                    to: Position {
                        row: movement.row,
                        col: movement.col,
                    },
                };
                if !self.branches.contains_key(&move_) {
                    let board = self
                        .board
                        .make_move(piece.row, piece.col, movement.row, movement.col)
                        .unwrap();
                    self.branches
                        .insert(move_.clone(), PredictionCache::new(board));
                }
                let cache = self.branches.get_mut(&move_).unwrap();
                let (color, depth) = match movement.movement_type {
                    MovementType::Free => (
                        match color {
                            Color::Black => Color::White,
                            Color::White => Color::Black,
                        },
                        depth - 1,
                    ),
                    MovementType::Forced => (color, depth),
                };
                if depth > 0 {
                    cache.calculate_moves(depth, color);
                }
            }
        }
    }

    fn get_score(&self, color: Color) -> u8 {
        if !self.branches.is_empty() {
            self.branches
                .values()
                .map(|p| p.get_score(color))
                .max()
                .unwrap_or(0)
        } else {
            if !self
                .board
                .get_movable_pieces(color)
                .collect::<Vec<_>>()
                .is_empty()
            {
                self.board
                    .pieces(color)
                    .collect::<Vec<_>>()
                    .len()
                    .try_into()
                    .unwrap()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Into<Vec<u8>> for Move {
    fn into(self) -> Vec<u8> {
        vec![
            u8::try_from(self.from.row).unwrap(),
            u8::try_from(self.from.col).unwrap(),
            u8::try_from(self.to.row).unwrap(),
            u8::try_from(self.to.col).unwrap(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct NoMoreMoves;

#[derive(Debug)]
pub struct InvalidMove;

#[cfg(test)]
mod tests;
