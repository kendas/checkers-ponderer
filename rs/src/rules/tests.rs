use super::*;
use crate::board::{Board, Color, Piece};

#[test]
fn non_king_white_free_movement() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 2);

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&2, &1)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&2, &3)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_friendly() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' w ' * '",
        "' * ' w ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 2);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&2, &1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_left_wall() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "w ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 0);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&2, &1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_right_wall() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' w",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 2, 7);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&1, &6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_enemy_and_right_wall() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' b",
        "* ' * ' * ' w '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 6);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&2, &5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_enemy_and_left_wall() {
    let board = make_board([
        "' * ' * ' * ' *",
        "b ' * ' * ' * '",
        "' w ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 2, 1);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&1, &2)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_forced_movement() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' b ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 2);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&1, &4)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_forced_movement_with_possibilities() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' b ' b ' *",
        "* ' * ' w ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 4);

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&1, &2)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&1, &6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' b ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 2);

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&4, &1)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&4, &3)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement_blocked_by_friendly() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' b ' * '",
        "' * ' b ' * ' *",
        "* ' b ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 4);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&4, &5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement_blocked_by_left_wall() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "b ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 0);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&4, &1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement_blocked_by_right_wall() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' b",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 4, 7);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&5, &6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_forced_movement() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' b ' * ' * '",
        "' w ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 3, 2);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&5, &0)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_free_movement() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' W ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 4, 3);

    assert_eq!(moves.len(), 4);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&3, &2)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&3, &4)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[2] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&5, &2)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[3] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&5, &4)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_free_movement_blocked_by_friendly() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' w ' *",
        "* ' * ' w ' * '",
        "' * ' W ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 4, 3);

    assert_eq!(moves.len(), 3);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&3, &2)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&5, &2)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[2] {
        Movement {
            movement_type: MovementType::Free,
            row,
            col,
        } => assert_eq!((row, col), (&5, &4)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_forced_movement() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' b ' * '",
        "' * ' W ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 4, 3);

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&2, &5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_forced_movement_multiple() {
    let board = make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' b ' * '",
        "' * ' W ' * ' *",
        "* ' b ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    let moves = get_moves(&board, 4, 3);

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&2, &5)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement {
            movement_type: MovementType::Forced,
            row,
            col,
        } => assert_eq!((row, col), (&6, &1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

/// Takes a board definition as an array of strings and creates a board.
///
/// "*" means a black square, "'" means a white square
/// "w" is a white piece, "W" is a white king,
/// "b" is a black piece, "B" is a black king.
///
/// # Examples
///
/// ```rust
/// let board = make_board([
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
/// ]);
/// assert_eq(board.piece_count(), 0);
/// ```
fn make_board(board: [&str; 8]) -> Board {
    let mut squares = [
        [None; 4], [None; 4], [None; 4], [None; 4], [None; 4], [None; 4], [None; 4], [None; 4],
    ];
    for (r, row) in board.iter().enumerate() {
        for (c, symbol) in row
            .split_ascii_whitespace()
            .enumerate()
            .filter(|(c, _)| (c + r + 1) % 2 == 0)
        {
            match symbol {
                "w" => {
                    squares[r][c / 2] = Some(Piece {
                        color: Color::White,
                        is_king: false,
                    })
                }
                "W" => {
                    squares[r][c / 2] = Some(Piece {
                        color: Color::White,
                        is_king: true,
                    })
                }
                "b" => {
                    squares[r][c / 2] = Some(Piece {
                        color: Color::Black,
                        is_king: false,
                    })
                }
                "B" => {
                    squares[r][c / 2] = Some(Piece {
                        color: Color::Black,
                        is_king: true,
                    })
                }
                _ => {}
            }
        }
    }
    Board::from(squares)
}
