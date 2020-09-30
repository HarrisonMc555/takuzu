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
    Ok(())
}

fn solve_surround_two_in_a_row_rows(board: &mut Board) -> Result<(), ()> {
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

            ensure_tile_equals(board.get_mut(row_index, column_index - 1), color.opposite())?;
            ensure_tile_equals(board.get_mut(row_index, column_index + 2), color.opposite())?;
        }
    }
    Ok(())
}

fn ensure_tile_equals(tile: Option<&mut Tile>, color: Color) -> Result<(), ()> {
    let tile = tile.ok_or(())?;
    if tile == &mut Tile::Filled(color.opposite()) {
        return Err(());
    }
    *tile = Tile::Filled(color);
    Ok(())
}
