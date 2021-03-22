use rules::MovementType;

use super::*;

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

    let black_piece = Piece {
        color: Color::Black,
        is_king: false,
    };
    let white_piece = Piece {
        color: Color::White,
        is_king: false,
    };
    let expected: Vec<(Piece, (u8, u8))> = vec![
        (black_piece, (0, 1)),
        (black_piece, (0, 3)),
        (black_piece, (0, 5)),
        (black_piece, (0, 7)), // 1st black row
        (black_piece, (1, 0)),
        (black_piece, (1, 2)),
        (black_piece, (1, 4)),
        (black_piece, (1, 6)), // 2nd black row
        (black_piece, (2, 1)),
        (black_piece, (2, 3)),
        (black_piece, (2, 5)),
        (black_piece, (2, 7)), // 3rd black row
        (white_piece, (5, 0)),
        (white_piece, (5, 2)),
        (white_piece, (5, 4)),
        (white_piece, (5, 6)), // 3rd white row
        (white_piece, (6, 1)),
        (white_piece, (6, 3)),
        (white_piece, (6, 5)),
        (white_piece, (6, 7)), // 2nd white row
        (white_piece, (7, 0)),
        (white_piece, (7, 2)),
        (white_piece, (7, 4)),
        (white_piece, (7, 6)), // 1st white row
    ];

    for (actual, expected) in board.all_pieces().chunks(4).zip(expected) {
        assert_eq!(actual[0], expected.0.color as u8);
        assert_eq!(actual[1], expected.0.is_king as u8);
        assert_eq!((actual[2], actual[3]), expected.1);
    }
}

#[test]
fn allows_iterating_over_white_pieces() {
    let board = Board::new();

    let white_piece = Piece {
        color: Color::White,
        is_king: false,
    };
    let expected: Vec<(Piece, (u8, u8))> = vec![
        (white_piece, (5, 0)),
        (white_piece, (5, 2)),
        (white_piece, (5, 4)),
        (white_piece, (5, 6)), // 3rd white row
        (white_piece, (6, 1)),
        (white_piece, (6, 3)),
        (white_piece, (6, 5)),
        (white_piece, (6, 7)), // 2nd white row
        (white_piece, (7, 0)),
        (white_piece, (7, 2)),
        (white_piece, (7, 4)),
        (white_piece, (7, 6)), // 1st white row
    ];

    for (actual, expected) in board.pieces(Color::White).chunks(4).zip(expected) {
        assert_eq!(actual[0], expected.0.color as u8);
        assert_eq!(actual[1], expected.0.is_king as u8);
        assert_eq!((actual[2], actual[3]), expected.1);
    }
}

#[test]
fn allows_iterating_over_black_pieces() {
    let board = Board::new();

    let black_piece = Piece {
        color: Color::Black,
        is_king: false,
    };
    let expected: Vec<(Piece, (u8, u8))> = vec![
        (black_piece, (0, 1)),
        (black_piece, (0, 3)),
        (black_piece, (0, 5)),
        (black_piece, (0, 7)), // 1st black row
        (black_piece, (1, 0)),
        (black_piece, (1, 2)),
        (black_piece, (1, 4)),
        (black_piece, (1, 6)), // 2nd black row
        (black_piece, (2, 1)),
        (black_piece, (2, 3)),
        (black_piece, (2, 5)),
        (black_piece, (2, 7)), // 3rd black row
    ];

    for (actual, expected) in board.pieces(Color::Black).chunks(4).zip(expected) {
        assert_eq!(actual[0], expected.0.color as u8);
        assert_eq!(actual[1], expected.0.is_king as u8);
        assert_eq!((actual[2], actual[3]), expected.1);
    }
}

#[test]
fn produces_valid_moves_for_a_starting_board() {
    let board = Board::new();

    let moves = board.moves_for(5, 2);
    let moves: Vec<_> = moves.chunks(3).collect();

    assert_eq!(moves.len(), 2);
    assert_eq!(
        (moves[0][0], moves[0][1], moves[0][2]),
        (MovementType::Free as u8, 4, 1)
    );
    assert_eq!(
        (moves[1][0], moves[1][1], moves[1][2]),
        (MovementType::Free as u8, 4, 3)
    );
}

#[test]
fn produces_valid_movable_pieces_for_a_staring_board() {
    let board = Board::new();

    let raw = board.get_movable_pieces(Color::White);
    let pieces: Vec<_> = raw.chunks(4).collect();

    assert_eq!(pieces.len(), 4);
    assert_eq!((pieces[0][2], pieces[0][3]), (5, 0));
    assert_eq!((pieces[1][2], pieces[1][3]), (5, 2));
    assert_eq!((pieces[2][2], pieces[2][3]), (5, 4));
    assert_eq!((pieces[3][2], pieces[3][3]), (5, 6));
}

#[test]
fn produces_only_forced_movable_pieces_if_some_are_available() {
    let mut board = Board::new();
    board.make_move(5, 0, 4, 1).unwrap();
    board.make_move(2, 3, 3, 2).unwrap();

    let raw = board.get_movable_pieces(Color::White);
    let pieces: Vec<_> = raw.chunks(4).collect();

    assert_eq!(pieces.len(), 1);
    assert_eq!((pieces[0][2], pieces[0][3]), (4, 1));
}

#[test]
fn attempting_to_move_empty_square_fails() {
    let mut board = Board::new();

    let result = board.make_move(4, 1, 3, 2);
    assert!(result.is_err());
}

#[test]
fn attempting_to_move_white_square_fails() {
    let mut board = Board::new();

    let result = board.make_move(3, 1, 2, 2);
    assert!(result.is_err());
}

#[test]
fn attempting_an_invalid_move_fails() {
    let mut board = Board::new();

    let result = board.make_move(5, 2, 2, 1);
    assert!(result.is_err());
}

#[test]
fn move_normally() {
    let mut board = Board::new();

    let result = board.make_move(5, 2, 4, 1);
    assert!(result.is_ok());

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

    assert!(board.get(2, 3).is_some());
    assert!(board.get(3, 2).is_none());
    assert!(board.get(4, 1).is_none());
    assert_eq!(board.count_pieces(Color::White), 12);
    assert_eq!(board.count_pieces(Color::Black), 11);
}

#[test]
fn moving_to_last_row_makes_king() {
    let white_piece = Some(Piece{color: Color::White, is_king: false});
    let black_piece = Some(Piece{color: Color::Black, is_king: false});
    let mut board = Board {
        squares: [
            [None; 4],
            [white_piece, None, None, None],
            [None; 4],
            [None; 4],
            [None; 4],
            [black_piece, None, None, None],
            [None; 4],
            [None; 4],
        ]
    };

    assert!(board.get(0, 1).is_none());
    assert!(board.get(1, 0).is_some());

    let result = board.make_move(1, 0, 0, 1);
    assert!(result.is_ok());

    assert!(board.get(0, 1).is_some());
    assert!(board.get(1, 0).is_none());
    assert_eq!(board.count_pieces(Color::White), 1);
    assert_eq!(board.count_pieces(Color::Black), 1);
    
    let piece = board.get(0, 1).unwrap();
    assert!(piece.is_king);
}
