#![allow(dead_code)]

use array2d::Array2D;

fn main() {
    let mut board = get_example();
    print_board(&board);
    println!();
    solve_surround_two_in_a_row(&mut board).unwrap();
    print_board(&board);
}

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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Axis {
    Row,
    Column,
}

const U: Tile = Tile::Unfilled;
const R: Tile = Tile::Filled(Color::Red);
const B: Tile = Tile::Filled(Color::Blue);

type Board = Array2D<Tile>;

fn get_example() -> Board {
    Array2D::from_rows(&vec![
        vec![U, B, U, U],
        vec![U, U, U, U],
        vec![U, B, B, U],
        vec![U, U, U, R],
    ])
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Unfilled => '_',
            Tile::Filled(color) => color.to_char(),
        }
    }
}

impl Color {
    fn to_char(self) -> char {
        match self {
            Color::Red => 'R',
            Color::Blue => 'B',
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
            print!("{}", tile.to_char())
        }
        println!();
    }
}

fn solve_surround_two_in_a_row(board: &mut Board) -> Result<(), ()> {
    solve_surround_two_in_a_row_rows(board)?;
    solve_surround_two_in_a_row_columns(board)?;
    Ok(())
}

fn solve_surround_two_in_a_row_rows(board: &mut Board) -> Result<(), ()> {
    for row_index in 0..board.num_rows() {
        for column_index in 0..(board.num_columns() - 1) {
            // let column_indices = (column_index, column_index + 1, column_index + 2);
            // two_in_a_row(board, row_index, column_indices, Axis::Row)?;
            let tile1 = board[(row_index, column_index)];
            let tile2 = board[(row_index, column_index + 1)];

            if tile1 != tile2 {
                continue;
            }

            let color = match tile1 {
                Tile::Filled(color) => color,
                _ => continue,
            };

            ensure_tile_equals(board.get_mut(row_index, column_index - 1), color.opposite())?;
            ensure_tile_equals(board.get_mut(row_index, column_index + 2), color.opposite())?;
        }
    }
    Ok(())
}

fn solve_surround_two_in_a_row_columns(board: &mut Board) -> Result<(), ()> {
    for column_index in 0..board.num_columns() {
        for row_index in 0..(board.num_rows() - 1) {
            // let column_indices = (column_index, column_index + 1, column_index + 2);
            // two_in_a_row(board, row_index, column_indices, Axis::Row)?;
            let tile1 = board[(row_index, column_index)];
            let tile2 = board[(row_index + 1, column_index)];

            if tile1 != tile2 {
                continue;
            }

            let color = match tile1 {
                Tile::Filled(color) => color,
                _ => continue,
            };

            ensure_tile_equals(board.get_mut(row_index - 1, column_index), color.opposite())?;
            ensure_tile_equals(board.get_mut(row_index + 2, column_index), color.opposite())?;
        }
    }
    Ok(())
}
// fn two_in_a_row(
//     board: &mut Board,
//     major_index: usize,
//     minor_indices: (usize, usize, usize),
//     axis: Axis,
// ) -> Result<(), ()> {
//     println!("two_in_a_row: board, {:?}, {:?}, {:?}", major_index, minor_indices, axis);
//     let (minor1, minor2, minor3) = minor_indices;
//     let color1: Color = get_tile(board, major_index, minor1, axis)
//         .and_then(|tile| tile.get_color())
//         .ok_or(())?;
//     let color2: Color = get_tile(board, major_index, minor2, axis)
//         .and_then(|tile| tile.get_color())
//         .ok_or(())?;
//     if color1 != color2 {
//         return Ok(());
//     };
//     let tile3 = get_tile(board, major_index, minor3, axis).ok_or(())?;
//     ensure_tile_equals(Some(tile3), color1.opposite())
// }

// fn get_tile(
//     board: &mut Board,
//     major_index: usize,
//     minor_index: usize,
//     axis: Axis,
// ) -> Option<&mut Tile> {
//     let (row, column) = match axis {
//         Axis::Row => (major_index, minor_index),
//         Axis::Column => (minor_index, major_index),
//     };
//     board.get_mut(row, column)
// }

fn ensure_tile_equals(tile: Option<&mut Tile>, color: Color) -> Result<(), ()> {
    let tile = match tile {
        Some(tile) => tile,
        Nothing => return Ok(()),
    };
    // let tile = tile.ok_or(())?;
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
