//! Representations and manipulations of the game.

use std::fmt::{self, Formatter, Display};

/// A 3x3 game board
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    /// The game board, as a row-major array. The “hole” is represented as `0`,
    /// and filled-in cells should be numbered `1` through `8`.
    cells: [u8; 9]
}

/// Converts from 2D coordinates in the grid to 1D indices into the board.
fn to_linear_index(ix: usize, iy: usize) -> usize {
    iy * 3 + ix
}

/// Converts from a 1D index into the board to a pair of `(x-index, y-index)`.
fn from_linear_index(i: usize) -> (usize, usize) {
    (i % 3, i / 3)
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let cells = &self.cells;
        write!(f, "{} {} {}
{} {} {}
{} {} {}", cells[0], cells[1], cells[2], cells[3], cells[4], cells[5], cells[6],
               cells[7], cells[8])
    }
}

/// The possible moves once can make.
///
/// `Move::Left` corresponds with swapping the board’s blank space with the
///  piece to its left, and so on.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
    Up,
    Down
}

pub const all_moves: [Move; 4] = [Move::Left, Move::Right, Move::Up, Move::Down];

impl Move {
    /// Returns the opposite move
    pub fn reverse(self) -> Self {
        match self {
            Move::Left => Move::Right,
            Move::Right => Move::Left,
            Move::Up => Move::Down,
            Move::Down => Move::Up
        }
    }
}

impl Board {
    /// Constructs a new `Board` by consuming the given array.
    /// The array must contain all integers from 0 to 8 exactly once, or `None`
    /// will be returned.
    pub fn from_array(cells: [u8; 9]) -> Option<Self> {
        let mut seen = [false; 9];

        for &cell in cells.into_iter() {
            if cell > 8 {
                return None;
            }

            seen[cell as usize] = true;
        }

        if seen.into_iter().all(|&b| b) {
            Some(Board{ cells: cells })
        }
        else {
            None
        }
    }

    /// Builds a new board from the given array, without checking to see if its
    /// prerequesite is met. Not tagged `unsafe` because there’s no *memory*
    /// unsafety, but it will absolutely violate application constraints; this 
    /// is why it’s only used in testing, to build expected results.
    #[cfg(test)]
    pub fn unsafe_from_array(cells: [u8; 9]) -> Self {
        Board { cells: cells }
    }

    /// Finds the 2D index of the hole, with the top left cell as `(0, 0)`.
    /// Panics if the hole isn’t found.
    fn hole_position(&self) -> (usize, usize) {
        let mut indices = (0, 0);

        for &cell in self.cells.into_iter() {
            let (ix, iy) = indices;

            if cell == 0 {
                return indices;
            }

            if ix == 2 {
                indices = (0, iy + 1);
            }
            else {
                indices = (ix + 1, iy);
            }
        }

        panic!("There was no hole in that board! Something went awfully awry…");
    }

    /// Verifies whether a `solution` really does change this board into the
    /// configuration given by `target`.
    pub fn verify(&self, target: &Self, solution: &[Move]) -> bool {
        let mut board = self.clone();
        for &play in solution {
            if let Some(new_board) = board.update(play) {
                board = new_board;
            }
            else {
                return false;
            }
        }

        return board == *target;
    }

    /// Estimates the cost to transform `self` into `goal`, measured in number
    /// of moves.
    ///
    /// This will deliberately be an underestimate, so it can be used in A*.
    pub fn estimate_cost(&self, goal: &Self) -> usize {
        let mut acc = 0;

        for tile in (1 .. 8) {
            acc += self.tile_distance(goal, tile)
        }

        acc
    }

    /// Computes the Manhattan distance of a tile from its destined place.
    fn tile_distance(&self, goal: &Self, for_tile: u8) -> usize {
        let (source_x, source_y) = self.tile_index(for_tile);
        let (goal_x, goal_y) = goal.tile_index(for_tile);
        
        (if source_x > goal_x { source_x - goal_x } else { goal_x - source_x })
        +
        (if source_y > goal_y { source_y - goal_y } else { goal_y - source_y })
    }

    fn tile_index(&self, tile: u8) -> (usize, usize) {
        from_linear_index(
            self.cells.iter().enumerate().filter(|&(_, &cell)| cell == tile)
                .next()
                .expect(&format!("The board didn’t have a {} tile", tile))
                .0)
    }

    /// Returns a new game board which is a copy of this one, except the blank
    /// space has been moved in the direction specified by `play`.
    ///
    /// If this would take the hole off the board, returns `None` instead.
    pub fn update(&self, command: Move) -> Option<Board> {
        let (ix, iy) = self.hole_position();

        match command {
            Move::Left => {
                if ix == 0 {
                    return None;
                }

                let mut new_cells = self.cells.clone();
                new_cells.swap(to_linear_index(ix, iy),
                               to_linear_index(ix - 1, iy));

                Some(Board { cells: new_cells })
            },
            Move::Right => {
                if ix == 2 {
                    return None;
                }

                let mut new_cells = self.cells.clone();
                new_cells.swap(to_linear_index(ix, iy),
                               to_linear_index(ix + 1, iy));

                Some(Board { cells: new_cells })
            },
            Move::Up => {
                if iy == 0 {
                    return None;
                }

                let mut new_cells = self.cells.clone();
                new_cells.swap(to_linear_index(ix, iy),
                               to_linear_index(ix, iy - 1));

                Some(Board { cells: new_cells })
            },
            Move::Down => {
                if iy == 2 {
                    return None;
                }

                let mut new_cells = self.cells.clone();
                new_cells.swap(to_linear_index(ix, iy),
                               to_linear_index(ix, iy + 1));

                Some(Board { cells: new_cells })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn trivial_board() -> Board {
        Board::unsafe_from_array([0, 1, 2, 3, 4, 5, 6, 7, 8])
    }

    #[test]
    fn trivial_board_construction() {
        let expected = trivial_board();

        assert_eq!(Some(expected),
                   Board::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    fn bad_boards_do_not_build() {
        assert_eq!(None, Board::from_array([1; 9]));
        assert_eq!(None, Board::from_array([0; 9]));
        assert_eq!(None, Board::from_array([1, 2, 3, 4, 5, 6, 5, 7, 0]));
    }

    #[test]
    fn trivial_move_right() {
        let expected = Board::unsafe_from_array([1, 0, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(Some(expected),
                   trivial_board().update(Move::Right));
    }

    #[test]
    fn bad_movements_fail() {
        let upper_left = trivial_board();

        assert_eq!(None, upper_left.update(Move::Left));
        assert_eq!(None, upper_left.update(Move::Up));

        let lower_right = Board::unsafe_from_array([1, 2, 3, 4, 5, 6, 7, 8, 0]);
        assert_eq!(None, lower_right.update(Move::Right));
        assert_eq!(None, lower_right.update(Move::Down));
    }
}
