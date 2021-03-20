use checkers_ponderer::{Board, Color};

#[test]
fn get_initial_moves() {
    let board = Board::new();

    let pieces = board.get_movable_pieces(Color::White);

    let expected_pieces = vec![
        // color, is_king, row, col
        [Color::White as u8, false as u8, 5u8, 1u8],
        [Color::White as u8, false as u8, 5, 3],
        [Color::White as u8, false as u8, 5, 5],
        [Color::White as u8, false as u8, 5, 7],
    ];

    assert_eq!(pieces.len(), 4 * 4);
    for (actual, expected) in pieces.chunks(4).zip(expected_pieces) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn make_the_first_move() {
    let mut board = Board::new();

    assert!(board.get(5, 3).is_some());
    assert!(board.get(4, 2).is_none());

    let result = board.make_move(5, 3, 4, 2);

    match result {
        Ok(()) => {}
        e => panic!("Unknown error {:?}", e),
    }
    assert!(board.get(5, 3).is_none());
    assert!(board.get(4, 2).is_some());
}
