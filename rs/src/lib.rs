mod board;
mod predictor;
mod rules;
mod utils;

use wasm_bindgen::prelude::*;

pub use board::{Color, GamePiece};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Board {
    board: crate::board::Board,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Board {
        Board {
            board: crate::board::Board::new(),
        }
    }
    pub fn count_pieces(&self, color: Color) -> usize {
        self.board.count_pieces(color)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<GamePiece> {
        self.board.get(row, col)
    }

    pub fn all_pieces(&self) -> Vec<u8> {
        self.board
            .all_pieces()
            .map(|p| p.into_vec())
            .flatten()
            .collect()
    }

    pub fn pieces(&self, color: Color) -> Vec<u8> {
        self.board
            .pieces(color)
            .map(|p| p.into_vec())
            .flatten()
            .collect()
    }

    pub fn moves_for(&self, row: usize, col: usize) -> Vec<u8> {
        self.board
            .moves_for(row, col)
            .into_iter()
            .map(|m| m.into_vec())
            .flatten()
            .collect()
    }

    pub fn get_movable_pieces(&self, color: Color) -> Vec<u8> {
        self.board
            .get_movable_pieces(color)
            .map(|p| p.into_vec())
            .flatten()
            .collect()
    }

    pub fn make_move(
        &mut self,
        from_row: u8,
        from_col: u8,
        to_row: u8,
        to_col: u8,
    ) -> Result<(), JsValue> {
        match self.board.make_move(
            from_row as usize,
            from_col as usize,
            to_row as usize,
            to_col as usize,
        ) {
            Ok(board) => {
                self.board = board;
                Ok(())
            }
            Err(_) => Err(JsValue::from_str("Invalid move")),
        }
    }
}

#[wasm_bindgen]
pub struct Predictor {
    predictor: crate::predictor::Predictor,
}

#[wasm_bindgen]
impl Predictor {
    #[wasm_bindgen(constructor)]
    pub fn new(board: Board, depth: u8, color: Color) -> Predictor {
        Predictor {
            predictor: predictor::Predictor::new(board.board.clone(), depth, color),
        }
    }

    pub fn get_next_move(&mut self) -> Result<Vec<u8>, JsValue> {
        match self.predictor.get_next_move() {
            Ok(move_) => Ok(move_.into()),
            Err(_) => Err(JsValue::from_str("No more moves")),
        }
    }

    pub fn register_move(
        &mut self,
        own_from_row: u8,
        own_from_col: u8,
        own_to_row: u8,
        own_to_col: u8,
        oponent_from_row: u8,
        oponent_from_col: u8,
        oponent_to_row: u8,
        oponent_to_col: u8,
    ) -> Result<(), JsValue> {
        let own_move = predictor::Move {
            from: predictor::Position {
                row: own_from_row as usize,
                col: own_from_col as usize,
            },
            to: predictor::Position {
                row: own_to_row as usize,
                col: own_to_col as usize,
            },
        };
        let oponent_move = predictor::Move {
            from: predictor::Position {
                row: oponent_from_row as usize,
                col: oponent_from_col as usize,
            },
            to: predictor::Position {
                row: oponent_to_row as usize,
                col: oponent_to_col as usize,
            },
        };
        match self.predictor.register_move(own_move, oponent_move) {
            Ok(_) => Ok(()),
            Err(_) => Err(JsValue::from_str("Invalid moves supplied"))
        }
    }
}
