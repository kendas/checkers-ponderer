use super::*;
use crate::rules::MovementType;
use crate::utils;

macro_rules! game_piece(
    ($color:expr, $king:expr, $row:expr, $col:expr) => (
        GamePiece {
            color: $color,
            is_king: $king,
            row: $row,
            col: $col,
        }
    )
);

#[test]
fn constructs_a_starting_board_with_12_pieces() {
    let board = Board::new();

    assert_eq!(board.count_pieces(Color::White), 12);
    assert_eq!(board.count_pieces(Color::Black), 12);

    let black_pieces = vec![
        (0, 1),
        (0, 3),
        (0, 5),
        (0, 7), // 1st row
        (1, 0),
        (1, 2),
        (1, 4),
        (1, 6), // 2nd row
        (2, 1),
        (2, 3),
        (2, 5),
        (2, 7), // 3rd row
    ];
    for position in black_pieces {
        let piece = board
            .get(position.0, position.1)
            .expect(&format!("Piece not found at {:?}", position));
        assert_eq!(piece.color, Color::Black);
    }

    let white_pieces = vec![
        (5, 0),
        (5, 2),
        (5, 4),
        (5, 6), // 3rd row
        (6, 1),
        (6, 3),
        (6, 5),
        (6, 7), // 2nd row
        (7, 0),
        (7, 2),
        (7, 4),
        (7, 6), // 1st row
    ];
    for (row, col) in white_pieces {
        let piece = board
            .get(row, col)
            .expect(&format!("Piece not found at {:?}", (row, col)));
        assert_eq!(piece.color, Color::White);
    }
}

#[test]
fn returns_none_for_white_squares() {
    let board = Board::new();

    let white_squares = vec![
        (0, 0),
        (0, 2),
        (0, 4),
        (0, 6), // 1st row
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7), // 2nd row
        (2, 0),
        (2, 2),
        (2, 4),
        (2, 6), // 3rd row
        (3, 1),
        (3, 3),
        (3, 5),
        (3, 7), // 4th row
        (4, 0),
        (4, 2),
        (4, 4),
        (4, 6), // 5th row
        (5, 1),
        (5, 3),
        (5, 5),
        (5, 7), // 6th row
        (6, 0),
        (6, 2),
        (6, 4),
        (6, 6), // 7th row
        (7, 1),
        (7, 3),
        (7, 5),
        (7, 7), // 8th row
    ];
    for position in white_squares {
        assert!(board.get(position.0, position.1).is_none())
    }
}

#[test]
fn allows_iterating_over_pieces() {
    let board = Board::new();

    let expected = vec![
        game_piece!(Color::Black, false, 0, 1),
        game_piece!(Color::Black, false, 0, 3),
        game_piece!(Color::Black, false, 0, 5),
        game_piece!(Color::Black, false, 0, 7), // 1st black row
        game_piece!(Color::Black, false, 1, 0),
        game_piece!(Color::Black, false, 1, 2),
        game_piece!(Color::Black, false, 1, 4),
        game_piece!(Color::Black, false, 1, 6), // 2nd black row
        game_piece!(Color::Black, false, 2, 1),
        game_piece!(Color::Black, false, 2, 3),
        game_piece!(Color::Black, false, 2, 5),
        game_piece!(Color::Black, false, 2, 7), // 3rd black row
        game_piece!(Color::White, false, 5, 0),
        game_piece!(Color::White, false, 5, 2),
        game_piece!(Color::White, false, 5, 4),
        game_piece!(Color::White, false, 5, 6), // 3rd white row
        game_piece!(Color::White, false, 6, 1),
        game_piece!(Color::White, false, 6, 3),
        game_piece!(Color::White, false, 6, 5),
        game_piece!(Color::White, false, 6, 7), // 2nd white row
        game_piece!(Color::White, false, 7, 0),
        game_piece!(Color::White, false, 7, 2),
        game_piece!(Color::White, false, 7, 4),
        game_piece!(Color::White, false, 7, 6), // 1st white row
    ];

    for (actual, expected) in board.all_pieces().into_iter().zip(expected) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn allows_iterating_over_white_pieces() {
    let board = Board::new();

    let expected = vec![
        game_piece!(Color::White, false, 5, 0),
        game_piece!(Color::White, false, 5, 2),
        game_piece!(Color::White, false, 5, 4),
        game_piece!(Color::White, false, 5, 6), // 3rd white row
        game_piece!(Color::White, false, 6, 1),
        game_piece!(Color::White, false, 6, 3),
        game_piece!(Color::White, false, 6, 5),
        game_piece!(Color::White, false, 6, 7), // 2nd white row
        game_piece!(Color::White, false, 7, 0),
        game_piece!(Color::White, false, 7, 2),
        game_piece!(Color::White, false, 7, 4),
        game_piece!(Color::White, false, 7, 6), // 1st white row
    ];

    for (actual, expected) in board.pieces(Color::White).zip(expected) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn allows_iterating_over_black_pieces() {
    let board = Board::new();

    let expected = vec![
        game_piece!(Color::Black, false, 0, 1),
        game_piece!(Color::Black, false, 0, 3),
        game_piece!(Color::Black, false, 0, 5),
        game_piece!(Color::Black, false, 0, 7), // 1st black row
        game_piece!(Color::Black, false, 1, 0),
        game_piece!(Color::Black, false, 1, 2),
        game_piece!(Color::Black, false, 1, 4),
        game_piece!(Color::Black, false, 1, 6), // 2nd black row
        game_piece!(Color::Black, false, 2, 1),
        game_piece!(Color::Black, false, 2, 3),
        game_piece!(Color::Black, false, 2, 5),
        game_piece!(Color::Black, false, 2, 7), // 3rd black row
    ];

    for (actual, expected) in board.pieces(Color::Black).zip(expected) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn produces_valid_moves_for_a_starting_board() {
    let board = Board::new();

    let moves = board.moves_for(5, 2);

    assert_eq!(moves.len(), 2);
    assert_eq!(
        moves[0],
        Movement {
            movement_type: MovementType::Free,
            row: 4,
            col: 1
        }
    );
    assert_eq!(
        moves[1],
        Movement {
            movement_type: MovementType::Free,
            row: 4,
            col: 3
        }
    );
}

#[test]
fn produces_valid_movable_pieces_for_a_staring_board() {
    let board = Board::new();

    let pieces: Vec<_> = board.get_movable_pieces(Color::White).collect();

    assert_eq!(pieces.len(), 4);
    assert_eq!(pieces[0], game_piece!(Color::White, false, 5, 0));
    assert_eq!(pieces[1], game_piece!(Color::White, false, 5, 2));
    assert_eq!(pieces[2], game_piece!(Color::White, false, 5, 4));
    assert_eq!(pieces[3], game_piece!(Color::White, false, 5, 6));
}

#[test]
fn produces_only_forced_movable_pieces_if_some_are_available() {
    let board = Board::new()
        .make_move(5, 0, 4, 1)
        .unwrap()
        .make_move(2, 3, 3, 2)
        .unwrap();

    let pieces: Vec<_> = board.get_movable_pieces(Color::White).collect();

    assert_eq!(pieces.len(), 1);
    assert_eq!(pieces[0], game_piece!(Color::White, false, 4, 1));
}

#[test]
fn attempting_to_move_empty_square_fails() {
    let board = Board::new();

    let result = board.make_move(4, 1, 3, 2);
    assert!(result.is_err());
}

#[test]
fn attempting_to_move_white_square_fails() {
    let board = Board::new();

    let result = board.make_move(3, 1, 2, 2);
    assert!(result.is_err());
}

#[test]
fn attempting_an_invalid_move_fails() {
    let board = Board::new();

    let result = board.make_move(5, 2, 2, 1);
    assert!(result.is_err());
}

#[test]
fn move_normally() {
    let board = Board::new();

    let result = board.make_move(5, 2, 4, 1);
    assert!(result.is_ok());
    let board = result.unwrap();

    assert!(board.get(5, 2).is_none());
    assert!(board.get(4, 1).is_some());
    assert_eq!(board.count_pieces(Color::White), 12);
    assert_eq!(board.count_pieces(Color::Black), 12);
}

#[test]
fn move_by_taking() {
    let mut board = Board::new();
    board.squares[5][0] = None;
    board.squares[4][0] = Some(Piece {
        color: Color::White,
        is_king: false,
    });
    board.squares[2][1] = None;
    board.squares[3][1] = Some(Piece {
        color: Color::Black,
        is_king: false,
    });

    assert!(board.get(5, 0).is_none());
    assert!(board.get(4, 1).is_some());
    assert!(board.get(2, 3).is_none());
    assert!(board.get(3, 2).is_some());

    let result = board.make_move(4, 1, 2, 3);
    assert!(result.is_ok());
    let board = result.unwrap();

    assert!(board.get(2, 3).is_some());
    assert!(board.get(3, 2).is_none());
    assert!(board.get(4, 1).is_none());
    assert_eq!(board.count_pieces(Color::White), 12);
    assert_eq!(board.count_pieces(Color::Black), 11);
}

#[test]
fn moving_to_last_row_makes_king() {
    let board = utils::make_board([
        "' * ' * ' * ' *",
        "w ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "b ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    assert!(board.get(0, 1).is_none());
    assert!(board.get(1, 0).is_some());

    let result = board.make_move(1, 0, 0, 1);
    assert!(result.is_ok());
    let board = result.unwrap();

    assert!(board.get(0, 1).is_some());
    assert!(board.get(1, 0).is_none());
    assert_eq!(board.count_pieces(Color::White), 1);
    assert_eq!(board.count_pieces(Color::Black), 1);

    let piece = board.get(0, 1).unwrap();
    assert!(piece.is_king);
}

#[test]
fn moving_from_last_row_remains_king() {
    let board = utils::make_board([
        "' W ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "b ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);

    assert!(board.get(0, 1).is_some());
    assert!(board.get(1, 0).is_none());

    let result = board.make_move(0, 1, 1, 0);
    assert!(result.is_ok());
    let board = result.unwrap();

    assert!(board.get(0, 1).is_none());
    assert!(board.get(1, 0).is_some());
    assert_eq!(board.count_pieces(Color::White), 1);
    assert_eq!(board.count_pieces(Color::Black), 1);

    let piece = board.get(1, 0).unwrap();
    assert!(piece.is_king);
}
