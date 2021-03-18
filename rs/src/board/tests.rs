use super::*;

#[test]
fn constructs_a_starting_board_with_12_pieces() {
    let board = Board::new();

    assert_eq!(board.count_pieces(Color::White), 12);
    assert_eq!(board.count_pieces(Color::Black), 12);

    let black_pieces = vec![
        Position(0, 0),
        Position(0, 2),
        Position(0, 4),
        Position(0, 6), // 1st row
        Position(1, 1),
        Position(1, 3),
        Position(1, 5),
        Position(1, 7), // 2nd row
        Position(2, 0),
        Position(2, 2),
        Position(2, 4),
        Position(2, 6), // 3rd row
    ];
    for position in black_pieces {
        let piece = board
            .get(position)
            .expect(&format!("Piece not found at {:?}", position));
        assert_eq!(piece.color, Color::Black);
    }

    let white_pieces = vec![
        Position(5, 1),
        Position(5, 3),
        Position(5, 5),
        Position(5, 7), // 3rd row
        Position(6, 0),
        Position(6, 2),
        Position(6, 4),
        Position(6, 6), // 2nd row
        Position(7, 1),
        Position(7, 3),
        Position(7, 5),
        Position(7, 7), // 1st row
    ];
    for position in white_pieces {
        let piece = board
            .get(position)
            .expect(&format!("Piece not found at {:?}", position));
        assert_eq!(piece.color, Color::White);
    }
}

#[test]
fn returns_none_for_white_squares() {
    let board = Board::new();

    let white_squares = vec![
        Position(0, 1),
        Position(0, 3),
        Position(0, 5),
        Position(0, 7), // 1st row
        Position(1, 0),
        Position(1, 2),
        Position(1, 4),
        Position(1, 6), // 2nd row
        Position(2, 1),
        Position(2, 3),
        Position(2, 5),
        Position(2, 7), // 3rd row
        Position(3, 0),
        Position(3, 2),
        Position(3, 4),
        Position(3, 6), // 4th row
        Position(4, 1),
        Position(4, 3),
        Position(4, 5),
        Position(4, 7), // 5th row
        Position(5, 0),
        Position(5, 2),
        Position(5, 4),
        Position(5, 6), // 6th row
        Position(6, 1),
        Position(6, 3),
        Position(6, 5),
        Position(6, 7), // 7th row
        Position(7, 0),
        Position(7, 2),
        Position(7, 4),
        Position(7, 6), // 8th row
    ];
    for position in white_squares {
        assert!(board.get(position).is_none())
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
    let expected = vec![
        (&black_piece, Position(0, 0)),
        (&black_piece, Position(0, 2)),
        (&black_piece, Position(0, 4)),
        (&black_piece, Position(0, 6)), // 1st black row
        (&black_piece, Position(1, 1)),
        (&black_piece, Position(1, 3)),
        (&black_piece, Position(1, 5)),
        (&black_piece, Position(1, 7)), // 2nd black row
        (&black_piece, Position(2, 0)),
        (&black_piece, Position(2, 2)),
        (&black_piece, Position(2, 4)),
        (&black_piece, Position(2, 6)), // 3rd black row
        (&white_piece, Position(5, 1)),
        (&white_piece, Position(5, 3)),
        (&white_piece, Position(5, 5)),
        (&white_piece, Position(5, 7)), // 3rd white row
        (&white_piece, Position(6, 0)),
        (&white_piece, Position(6, 2)),
        (&white_piece, Position(6, 4)),
        (&white_piece, Position(6, 6)), // 2nd white row
        (&white_piece, Position(7, 1)),
        (&white_piece, Position(7, 3)),
        (&white_piece, Position(7, 5)),
        (&white_piece, Position(7, 7)), // 1st white row
    ];

    for (expected, actual) in board.all_pieces().into_iter().zip(expected) {
        assert_eq!(expected.0, actual.0);
        assert_eq!(expected.1, actual.1);
    }
}

#[test]
fn allows_iterating_over_white_pieces() {
    let board = Board::new();

    let white_piece = Piece {
        color: Color::White,
        is_king: false,
    };
    let expected = vec![
        (&white_piece, Position(5, 1)),
        (&white_piece, Position(5, 3)),
        (&white_piece, Position(5, 5)),
        (&white_piece, Position(5, 7)), // 3rd white row
        (&white_piece, Position(6, 0)),
        (&white_piece, Position(6, 2)),
        (&white_piece, Position(6, 4)),
        (&white_piece, Position(6, 6)), // 2nd white row
        (&white_piece, Position(7, 1)),
        (&white_piece, Position(7, 3)),
        (&white_piece, Position(7, 5)),
        (&white_piece, Position(7, 7)), // 1st white row
    ];

    for (expected, actual) in board.pieces(Color::White).into_iter().zip(expected) {
        assert_eq!(expected.0, actual.0);
        assert_eq!(expected.1, actual.1);
    }
}

#[test]
fn allows_iterating_over_black_pieces() {
    let board = Board::new();

    let black_piece = Piece {
        color: Color::Black,
        is_king: false,
    };
    let expected = vec![
        (&black_piece, Position(0, 0)),
        (&black_piece, Position(0, 2)),
        (&black_piece, Position(0, 4)),
        (&black_piece, Position(0, 6)), // 1st black row
        (&black_piece, Position(1, 1)),
        (&black_piece, Position(1, 3)),
        (&black_piece, Position(1, 5)),
        (&black_piece, Position(1, 7)), // 2nd black row
        (&black_piece, Position(2, 0)),
        (&black_piece, Position(2, 2)),
        (&black_piece, Position(2, 4)),
        (&black_piece, Position(2, 6)), // 3rd black row
    ];

    for (expected, actual) in board.pieces(Color::Black).into_iter().zip(expected) {
        assert_eq!(expected.0, actual.0);
        assert_eq!(expected.1, actual.1);
    }
}

#[test]
fn produces_valid_moves_for_a_starting_board() {
    let board = Board::new();

    let moves = board.moves_for(Position(5, 1));

    assert_eq!(moves.len(), 2);
    match &moves[0] {
        Movement::Free(position) => assert_eq!(position, &Position(4, 0)),
        other => panic!("Unexpected move {:?}", other),
    }
    match &moves[1] {
        Movement::Free(position) => assert_eq!(position, &Position(4, 2)),
        other => panic!("Unexpected move {:?}", other),
    }
}

#[test]
fn produces_valid_movable_pieces_for_a_staring_board() {
    let board = Board::new();

    let pieces = board.get_movable_pieces(Color::White);

    assert_eq!(pieces.len(), 4);
    assert_eq!(pieces[0], Position(5, 1));
    assert_eq!(pieces[1], Position(5, 3));
    assert_eq!(pieces[2], Position(5, 5));
    assert_eq!(pieces[3], Position(5, 7));
}
