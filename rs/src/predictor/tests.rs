use super::*;
use crate::{utils, Color};

macro_rules! movement {
    ($fr:expr, $fc:expr, $tr:expr, $tc:expr) => {
        Move {
            from: Position { row: $fr, col: $fc },
            to: Position { row: $tr, col: $tc },
        }
    };
}

#[test]
fn calculates_the_first_move_with_lookahead_of_one() {
    let board = utils::make_board([
        "' b ' b ' b ' b",
        "b ' b ' b ' b '",
        "' b ' b ' b ' b",
        "* ' * ' * ' * '",
        "' * ' w ' * ' *",
        "w ' * ' w ' w '",
        "' w ' w ' w ' w",
        "w ' w ' w ' w '",
    ]);
    let mut predictor = Predictor::new(board, 1, Color::Black);

    let actual = predictor.get_next_move().unwrap();
    let possibles = vec![
        movement!(2, 1, 3, 0),
        movement!(2, 1, 3, 2),
        movement!(2, 3, 3, 2),
        movement!(2, 3, 3, 4),
        movement!(2, 5, 3, 4),
        movement!(2, 5, 3, 6),
        movement!(2, 7, 3, 6),
    ];
    let mut found = false;
    for possible in possibles {
        if actual == possible {
            found = true;
            break;
        }
    }
    if !found {
        panic!("The answer {:?} not in possible moves", actual);
    }
}

#[test]
fn calculates_the_non_stuck_move_with_lookahead_of_one() {
    let board = utils::make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' b '",
        "' * ' * ' * ' *",
        "* ' * ' w ' w '",
        "' * ' * ' w ' w",
        "* ' * ' * ' * '",
    ]);
    let mut predictor = Predictor::new(board, 1, Color::Black);

    let actual = predictor.get_next_move().unwrap();
    let expected = movement!(3, 6, 4, 5);
    assert_eq!(actual, expected);
}

#[test]
fn calculates_the_optimal_move_with_lookahead_of_two() {
    let board = utils::make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' b ' * '",
        "' * ' * ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);
    let mut predictor = Predictor::new(board, 2, Color::Black);

    let actual = predictor.get_next_move().unwrap();
    let expected = movement!(3, 4, 4, 5);
    assert_eq!(actual, expected);
}

#[test]
fn can_be_given_moves() {
    let board = utils::make_board([
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' b ' * '",
        "' * ' * ' * ' *",
        "* ' w ' * ' * '",
        "' * ' * ' * ' *",
        "* ' * ' * ' * '",
    ]);
    let mut predictor = Predictor::new(board, 2, Color::Black);

    let actual = predictor.get_next_move().unwrap();
    let expected = movement!(3, 4, 4, 5);
    assert_eq!(actual, expected);

    predictor.register_move(actual, movement!(5, 2, 4, 3)).unwrap();

    let actual = predictor.get_next_move().unwrap();
    let possibles = vec![
        movement!(4, 5, 5, 4),
        movement!(4, 5, 5, 6),
    ];
    let mut found = false;
    for possible in possibles {
        if actual == possible {
            found = true;
            break;
        }
    }
    if !found {
        panic!("The answer {:?} not in possible moves", actual);
    }
}
