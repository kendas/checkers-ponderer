use super::*;
use crate::board::{Board, Color, Piece, Position};

#[test]
fn non_king_white_free_movement() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 2));

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 1)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 3)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_friendly() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' w ' * '",
        "' * ' w ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 2));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_left_wall() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "w ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 0));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_right_wall() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' w",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(3, 7));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(2, 6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_enemy_and_right_wall() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' b",
        "* ' * ' * ' w '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 6));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_free_movement_blocked_by_enemy_and_left_wall() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "b ' * ' * ' * '",
        "' w ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(3, 1));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(2, 2)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_forced_movement() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' b ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 2));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Forced(position) => assert_eq!(position, &Position(2, 4)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_white_forced_movement_with_possibilities() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' b ' b ' *",
        "* ' * ' w ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 4));

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement::Forced(position) => assert_eq!(position, &Position(2, 2)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Forced(position) => assert_eq!(position, &Position(2, 6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' b ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 2));

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 1)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 3)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement_blocked_by_friendly() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' b ' * '",
        "' * ' b ' * ' *",
        "* ' b ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(2, 4));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement_blocked_by_left_wall() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "b ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 0));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_free_movement_blocked_by_right_wall() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' b",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(3, 7));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(4, 6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn non_king_black_forced_movement() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' b ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(3, 3));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Forced(position) => assert_eq!(position, &Position(5, 1)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_free_movement() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' W ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 4));

    assert_eq!(moves.len(), 4);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 3)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 5)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[2] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 3)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[3] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_free_movement_blocked_by_friendly() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' w '",
        "' * ' * ' w ' *",
        "* ' * ' W ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 4));

    assert_eq!(moves.len(), 3);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(3, 3)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 3)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[2] {
        Movement::Free(position) => assert_eq!(position, &Position(5, 5)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_forced_movement() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' b ' *",
        "* ' * ' W ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 4));

    assert_eq!(moves.len(), 1);
    match &moves[0] {
        Movement::Forced(position) => assert_eq!(position, &Position(2, 6)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn king_white_forced_movement_multiple() {
    let board = make_board([
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' b ' *",
        "* ' * ' W ' * '",
        "' * ' b ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
    ]);

    let moves = get_moves(&board, Position(4, 4));

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement::Forced(position) => assert_eq!(position, &Position(2, 6)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Forced(position) => assert_eq!(position, &Position(6, 2)),
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
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
///     "* ' * ' * ' * '",
///     "' * ' * ' * ' *",
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
            .filter(|(c, _)| (c + r) % 2 == 0)
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
