use crate::board::{Board, Color, Piece, Position};

#[derive(Debug)]
pub enum Movement {
    Free(Position),
    Forced(Position),
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

pub fn get_moves(board: &Board, position: Position) -> Vec<Movement> {
    match board.get(position) {
        Some(piece) => {
            let possibilities = get_possibilities(&piece);

            let mut moves = vec![];
            for (row_direction, col_direction) in possibilities {
                if let Some(position) = get_next(position, row_direction, col_direction) {
                    match board.get(position) {
                        None => moves.push(Movement::Free(position)),
                        Some(other_piece) => {
                            if other_piece.color != piece.color {
                                if let Some(position) =
                                    get_next(position, row_direction, col_direction)
                                {
                                    if board.get(position).is_none() {
                                        moves.push(Movement::Forced(position));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if moves.iter().any(|m| matches!(m, Movement::Forced(_))) {
                moves
                    .into_iter()
                    .filter(|m| matches!(m, Movement::Forced(_)))
                    .collect()
            } else {
                moves
            }
        }
        None => Default::default(),
    }
}

fn get_next(
    position: Position,
    row_direction: Direction,
    col_direction: Direction,
) -> Option<Position> {
    use Direction::*;
    if matches!(row_direction, Decrease) && position.0 <= 0 {
        None
    } else if matches!(row_direction, Increase) && position.0 >= 7 {
        None
    } else if matches!(col_direction, Decrease) && position.1 <= 0 {
        None
    } else if matches!(col_direction, Increase) && position.1 >= 7 {
        None
    } else {
        let row = match row_direction {
            Increase => position.0 + 1,
            Decrease => position.0 - 1,
        };
        let col = match col_direction {
            Increase => position.1 + 1,
            Decrease => position.1 - 1,
        };
        Some(Position(row, col))
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
