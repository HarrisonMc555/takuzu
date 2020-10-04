#![allow(dead_code)]

use array2d::Array2D;

fn main() {
    let mut board = get_example_4();
    solve_board(&mut board).unwrap();
    let mut board = get_example_6();
    solve_board(&mut board).unwrap();
    let mut board = get_example_8();
    solve_board(&mut board).unwrap();
    let mut board = get_example_10();
    solve_board(&mut board).unwrap();
    let mut board = get_example_12();
    solve_board(&mut board).unwrap();
}

type Answer = Result<(), ()>;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Unfilled,
    Filled(Color),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Color {
    Red,
    Blue,
}

const U: Tile = Tile::Unfilled;
const R: Tile = Tile::Filled(Color::Red);
const B: Tile = Tile::Filled(Color::Blue);

type Board = Array2D<Tile>;

fn get_example_4() -> Board {
    Array2D::from_rows(&[
        vec![U, B, U, U],
        vec![U, U, U, U],
        vec![U, B, B, U],
        vec![U, U, U, R],
    ])
}

fn get_example_6() -> Board {
    Array2D::from_rows(&[
        vec![R, U, U, B, U, B],
        vec![U, U, U, U, U, U],
        vec![R, R, U, B, U, U],
        vec![U, R, U, U, U, U],
        vec![U, U, U, B, U, U],
        vec![U, U, U, U, R, U],
    ])
}

fn get_example_8() -> Board {
    Array2D::from_rows(&[
        vec![R, U, U, B, U, U, U, U],
        vec![U, U, U, B, U, U, U, R],
        vec![U, R, U, U, U, U, U, U],
        vec![U, U, B, U, U, U, R, R],
        vec![B, U, U, B, B, U, U, U],
        vec![U, B, U, U, U, U, R, U],
        vec![U, U, U, U, U, U, U, R],
        vec![B, U, B, U, B, U, U, U],
    ])
}

fn get_example_10() -> Board {
    Array2D::from_rows(&[
        vec![R, U, U, U, U, U, U, U, U, U],
        vec![U, U, U, U, B, B, U, U, R, B],
        vec![U, U, B, U, U, U, U, U, U, B],
        vec![U, U, B, U, R, U, B, U, U, U],
        vec![U, R, U, U, U, U, U, U, U, U],
        vec![U, B, U, U, U, U, R, U, R, U],
        vec![B, U, U, R, B, U, U, U, B, U],
        vec![U, U, B, U, U, U, U, B, U, R],
        vec![U, R, U, U, R, U, U, U, R, U],
        vec![U, U, B, B, U, U, B, U, U, R],
    ])
}

fn get_example_12() -> Board {
    Array2D::from_rows(&[
        vec![U, U, U, R, U, B, R, B, U, R, R, U],
        vec![B, U, U, R, U, U, U, U, U, U, U, U],
        vec![U, U, B, B, U, U, B, U, U, B, U, U],
        vec![U, R, U, U, U, U, U, U, U, B, B, U],
        vec![U, U, U, R, U, U, B, U, U, U, B, R],
        vec![U, R, B, U, U, U, U, B, U, U, U, U],
        vec![U, U, U, R, R, U, U, B, U, U, R, B],
        vec![U, U, U, U, U, B, R, U, U, U, U, U],
        vec![U, U, R, U, U, U, U, U, U, U, U, U],
        vec![U, U, U, R, U, U, U, U, B, R, U, B],
        vec![U, R, U, U, U, U, B, U, U, U, B, U],
        vec![R, R, U, U, U, U, B, U, R, U, U, B],
    ])
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Unfilled => '_',
            Tile::Filled(color) => color.to_char(),
        }
    }

    fn get_color(self) -> Option<Color> {
        match self {
            Tile::Unfilled => None,
            Tile::Filled(color) => Some(color),
        }
    }
}

impl Color {
    fn to_char(self) -> char {
        match self {
            Color::Red => '+',
            Color::Blue => 'o',
        }
    }

    fn opposite(self) -> Color {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
        }
    }
}

fn print_board(board: &Board) {
    for row in board.rows_iter() {
        for tile in row {
            print!("{} ", tile.to_char())
        }
        println!();
    }
}

fn solve_board(board: &mut Board) -> Answer {
    loop {
        print_board(board);
        let board_copy = board.clone();
        // print_board(board);
        // println!();
        solve_surround_two_in_a_row(board)?;
        // print_board(board);
        // println!();
        solve_fill_in_between_two(board)?;
        // print_board(board);
        // println!();
        solve_equal_colors(board)?;
        // print_board(board);
        // println!();
        solve_no_two_lines_same(board)?;
        println!();
        if board == &board_copy {
            break;
        }
    }
    Ok(())
}

fn solve_surround_two_in_a_row(board: &mut Board) -> Answer {
    solve_surround_two_in_a_row_rows(board)?;
    solve_surround_two_in_a_row_columns(board)?;
    Ok(())
}

fn solve_surround_two_in_a_row_rows(board: &mut Board) -> Answer {
    for row_index in 0..board.num_rows() {
        for column_index in 0..(board.num_columns() - 1) {
            let tile1 = board[(row_index, column_index)];
            let tile2 = board[(row_index, column_index + 1)];

            if tile1 != tile2 {
                continue;
            }

            let color = match tile1 {
                Tile::Filled(color) => color,
                _ => continue,
            };

            if column_index > 0 {
                ensure_tile_equals(board.get_mut(row_index, column_index - 1), color.opposite())?;
            }
            ensure_tile_equals(board.get_mut(row_index, column_index + 2), color.opposite())?;
        }
    }
    Ok(())
}

fn solve_surround_two_in_a_row_columns(board: &mut Board) -> Answer {
    for column_index in 0..board.num_columns() {
        for row_index in 0..(board.num_rows() - 1) {
            let tile1 = board[(row_index, column_index)];
            let tile2 = board[(row_index + 1, column_index)];

            if tile1 != tile2 {
                continue;
            }

            let color = match tile1 {
                Tile::Filled(color) => color,
                _ => continue,
            };

            if row_index > 0 {
                ensure_tile_equals(board.get_mut(row_index - 1, column_index), color.opposite())?;
            }
            ensure_tile_equals(board.get_mut(row_index + 2, column_index), color.opposite())?;
        }
    }
    Ok(())
}

fn solve_fill_in_between_two(board: &mut Board) -> Answer {
    solve_fill_in_between_two_rows(board)?;
    solve_fill_in_between_two_columns(board)?;
    Ok(())
}

fn solve_fill_in_between_two_rows(board: &mut Board) -> Answer {
    for row_index in 0..board.num_rows() {
        for column_index in 0..(board.num_columns() - 2) {
            let tile1 = board[(row_index, column_index)];
            let tile3 = board[(row_index, column_index + 2)];

            if tile1 != tile3 {
                continue;
            }

            let color = match tile1 {
                Tile::Filled(color) => color,
                _ => continue,
            };

            ensure_tile_equals(board.get_mut(row_index, column_index + 1), color.opposite())?;
        }
    }
    Ok(())
}

fn solve_fill_in_between_two_columns(board: &mut Board) -> Answer {
    for column_index in 0..board.num_columns() {
        for row_index in 0..(board.num_rows() - 2) {
            let tile1 = board[(row_index, column_index)];
            let tile3 = board[(row_index + 2, column_index)];

            if tile1 != tile3 {
                continue;
            }

            let color = match tile1 {
                Tile::Filled(color) => color,
                _ => continue,
            };

            ensure_tile_equals(board.get_mut(row_index + 1, column_index), color.opposite())?;
        }
    }
    Ok(())
}

fn solve_equal_colors(board: &mut Board) -> Answer {
    solve_equal_colors_rows(board, Color::Red)?;
    solve_equal_colors_rows(board, Color::Blue)?;
    solve_equal_colors_columns(board, Color::Red)?;
    solve_equal_colors_columns(board, Color::Blue)?;
    Ok(())
}

fn solve_equal_colors_rows(board: &mut Board, color: Color) -> Answer {
    for row_index in 0..board.num_rows() {
        let max_tiles_of_one_color = board.row_len() / 2;
        let num_of_this_color = board
            .row_iter(row_index)
            .filter(|tile| tile == &&Tile::Filled(color))
            .count();
        if num_of_this_color > max_tiles_of_one_color {
            return Err(()); // Too many
        } else if num_of_this_color == max_tiles_of_one_color {
            for column_index in 0..board.num_columns() {
                let tile = board.get_mut(row_index, column_index).ok_or(())?;
                if tile == &mut Tile::Unfilled {
                    *tile = Tile::Filled(color.opposite());
                }
            }
        }
    }
    Ok(())
}

fn solve_equal_colors_columns(board: &mut Board, color: Color) -> Answer {
    for column_index in 0..board.num_columns() {
        let max_tiles_of_one_color = board.column_len() / 2;
        let num_of_this_color = board
            .column_iter(column_index)
            .filter(|tile| tile == &&Tile::Filled(color))
            .count();
        if num_of_this_color > max_tiles_of_one_color {
            return Err(()); // Too many
        } else if num_of_this_color == max_tiles_of_one_color {
            for row_index in 0..board.num_rows() {
                let tile = board.get_mut(row_index, column_index).ok_or(())?;
                if tile == &mut Tile::Unfilled {
                    *tile = Tile::Filled(color.opposite());
                }
            }
        }
    }
    Ok(())
}

fn solve_no_two_lines_same(board: &mut Board) -> Answer {
    solve_no_two_lines_same_rows(board)?;
    solve_no_two_lines_same_columns(board)?;
    Ok(())
}

fn solve_no_two_lines_same_rows(board: &mut Board) -> Answer {
    let mut filled_unfilled_indices = None;
    for filled_index in 0..board.num_rows() {
        let num_unfilled = board
            .row_iter(filled_index)
            .filter(|tile| tile == &&Tile::Unfilled)
            .count();
        if num_unfilled != 0 {
            continue;
        }
        let filled_row = board.row_iter(filled_index).collect::<Vec<_>>();
        for unfilled_index in (0..board.num_rows()).filter(|&i| i != filled_index) {
            let num_unfilled = board
                .row_iter(unfilled_index)
                .filter(|tile| tile == &&Tile::Unfilled)
                .count();
            // We want exactly 2 to be unfilled
            if num_unfilled != 2 {
                continue;
            }
            let num_different = filled_row
                .iter()
                .zip(board.row_iter(unfilled_index))
                .filter(|(tile1, tile2)| tile1 != &tile2)
                .count();
            // There are at least two different because the filled row has no unfilled tiles, while
            // the filled row has exactly 2 unfilled tiles. If there are more than 2 different, then
            // they don't match
            if num_different != 2 {
                continue;
            }
            filled_unfilled_indices = Some((filled_index, unfilled_index));
        }
    }

    if let Some((filled_index, unfilled_index)) = filled_unfilled_indices {
        for column_index in 0..board.row_len() {
            if board[(unfilled_index, column_index)] == Tile::Unfilled {
                board[(unfilled_index, column_index)] = Tile::Filled(
                    board[(filled_index, column_index)]
                        .get_color()
                        .unwrap()
                        .opposite(),
                );
            }
        }
    }

    Ok(())
}

fn solve_no_two_lines_same_columns(board: &mut Board) -> Answer {
    let mut filled_unfilled_indices = Vec::new();
    for filled_index in 0..board.num_columns() {
        let num_unfilled = board
            .column_iter(filled_index)
            .filter(|tile| tile == &&Tile::Unfilled)
            .count();
        if num_unfilled != 0 {
            continue;
        }
        let filled_column = board.column_iter(filled_index).collect::<Vec<_>>();
        for unfilled_index in (0..board.num_columns()).filter(|&i| i != filled_index) {
            let num_unfilled = board
                .column_iter(unfilled_index)
                .filter(|tile| tile == &&Tile::Unfilled)
                .count();
            // We want exactly 2 to be unfilled
            if num_unfilled != 2 {
                continue;
            }
            let num_different = filled_column
                .iter()
                .zip(board.column_iter(unfilled_index))
                .filter(|(tile1, tile2)| tile1 != &tile2)
                .count();
            // There are at least two different because the filled column has no unfilled tiles, while
            // the filled column has exactly 2 unfilled tiles. If there are more than 2 different, then
            // they don't match
            if num_different != 2 {
                continue;
            }
            filled_unfilled_indices.push((filled_index, unfilled_index));
        }
    }

    for (filled_index, unfilled_index) in filled_unfilled_indices {
        for row_index in 0..board.column_len() {
            if board[(row_index, unfilled_index)] == Tile::Unfilled {
                board[(row_index, unfilled_index)] = Tile::Filled(
                    board[(row_index, filled_index)]
                        .get_color()
                        .unwrap()
                        .opposite(),
                );
            }
        }
    }

    Ok(())
}

fn ensure_tile_equals(tile: Option<&mut Tile>, color: Color) -> Answer {
    let tile = match tile {
        Some(tile) => tile,
        None => return Ok(()),
    };
    if tile == &mut Tile::Filled(color.opposite()) {
        println!(
            "Tile was {:?} but was supposed to be {:?}",
            color.opposite(),
            color
        );
        return Err(());
    }
    *tile = Tile::Filled(color);
    Ok(())
}
