#![allow(dead_code)]

use array2d::Array2D;

fn main() {
    let board = get_example();
    print_board(&board);
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Unfilled,
    Red,
    Blue,
}

const U: Tile = Tile::Unfilled;
const R: Tile = Tile::Red;
const B: Tile = Tile::Blue;

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
            Tile::Red => 'R',
            Tile::Blue => 'B',
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
