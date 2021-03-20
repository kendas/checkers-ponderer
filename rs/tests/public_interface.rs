use checkers_ponderer::{Board, Color};

#[test]
fn get_initial_moves() {
    let board = Board::new();

    let pieces = board.get_movable_pieces(Color::White);

    let expected_pieces = vec![
        // color, is_king, row, col
        [Color::White as u8, false as u8, 5u8, 0u8],
        [Color::White as u8, false as u8, 5, 2],
        [Color::White as u8, false as u8, 5, 4],
        [Color::White as u8, false as u8, 5, 6],
    ];

    assert_eq!(pieces.len(), 4 * 4);
    for (actual, expected) in pieces.chunks(4).zip(expected_pieces) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn make_the_first_move() {
    let mut board = Board::new();

    assert!(board.get(5, 2).is_some());
    assert!(board.get(4, 1).is_none());

    let result = board.make_move(5, 2, 4, 1);

    match result {
        Ok(()) => {}
        e => panic!("Unknown error {:?}", e),
    }
    assert!(board.get(5, 2).is_none());
    assert!(board.get(4, 1).is_some());
}

#[test]
fn make_the_second_move() {
    let mut board = Board::new();
    board.make_move(5, 2, 4, 1).unwrap();

    assert!(board.get(3, 2).is_none());
    assert!(board.get(2, 3).is_some());
    let result = board.make_move(2, 3, 3, 2);
    match result {
        Ok(()) => {}
        e => panic!("Unknown error {:?}", e),
    }
    assert!(board.get(5, 2).is_none());
    assert!(board.get(4, 1).is_some());
}

#[test]
fn take_a_piece() {
    let mut board = Board::new();
    board.make_move(5, 2, 4, 1).unwrap();
    board.make_move(2, 3, 3, 2).unwrap();

    assert!(board.get(3, 2).is_some());
    assert!(board.get(4, 1).is_some());
    assert!(board.get(2, 3).is_none());
    let result = board.make_move(4, 1, 2, 3);
    match result {
        Ok(()) => {}
        e => panic!("Unknown error {:?}", e),
    }
    assert!(board.get(3, 2).is_none());
    assert!(board.get(4, 1).is_none());
    assert!(board.get(2, 3).is_some());
}

#[test]
fn get_pieces() {
    let board = Board::new();

    for row in 0..8 {
        for col in ((row + 1) % 2..8).step_by(2) {
            let piece = board.get(row, col);
            if row > 2 && row < 5 {
                assert!(piece.is_none())
            } else {
                assert!(piece.is_some())
            }
        }
    }
}

#[test]
fn get_movable_pieces() {
    let board = Board::new();

    let pieces = board.get_movable_pieces(Color::White);
    let pieces: Vec<_> = pieces.chunks(4).collect();

    assert_eq!(pieces.len(), 4);
    for piece in pieces {
        assert_eq!(piece[0], Color::White as u8);
        assert_eq!(piece[1], false as u8);
    }
}
