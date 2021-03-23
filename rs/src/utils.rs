use crate::board::{Board, Color, Piece};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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
#[cfg(test)]
pub(crate) fn make_board(board: [&str; 8]) -> Board {
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
