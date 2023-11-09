use std::collections::{HashSet};

#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub uneaten_squares: HashSet<(usize, usize)>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut uneaten_squares: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..width {
            for j in 0..height {
                uneaten_squares.insert((i, j));
            }
        }
        Board {
            width,
            height,
            uneaten_squares,
        }
    }

    pub fn print(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                if self.uneaten_squares.contains(&(i, j)) {
                    print!("O ");
                } else {
                    print!("X ");
                }
            }
            println!(); // Move to the next line after each row.
        }
    }

    pub fn chomp(&mut self, row: usize, col: usize) {
        for i in row..self.width {
            for j in col..self.height {
                self.uneaten_squares.remove(&(i, j));
            }
        }
    }

    pub fn winning_move(&self) -> Option<(usize, usize)> {
        // Check if the upper-left square is the only one left.
        if self.uneaten_squares.len() == 1 && self.uneaten_squares.contains(&(0, 0)) {
            None
        } else {
            // Iterate through remaining rows and columns.
            for i in 0..self.width {
                for j in 0..self.height {
                    // Skip the upper-left square.
                    if i == 0 && j == 0 {
                        continue;
                    }

                    // Check if the square has already been chomped.
                    if !self.uneaten_squares.contains(&(i, j)) {
                        continue;
                    }

                    // Create a copy of the board.
                    let mut temp_board = self.clone();

                    // Perform the chomp operation on the copy.
                    temp_board.chomp(i, j);

                    // Recursively call winning_move on the copy.
                    if temp_board.winning_move().is_none() {
                        return Some((i, j));
                    }
                }
            }

            None // No winning move found.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new(3, 3);

        assert_eq!(board.width, 3);
        assert_eq!(board.height, 3);
        assert_eq!(board.uneaten_squares.len(), 9);
        // Add assertions for other fields if needed
    }

    #[test]
    fn test_chomp() {
        let mut board = Board::new(3, 3);
        board.chomp(1, 1);

        // On a 3x3 board, afer chomping (1, 1) there should be 5 squares left including the poisened square.
        assert_eq!(board.uneaten_squares.len(), 5);

        // After chomping at (1, 1), squares below and to the right should be removed
        assert!(!board.uneaten_squares.contains(&(1, 1)));
        assert!(!board.uneaten_squares.contains(&(1, 2)));
        assert!(!board.uneaten_squares.contains(&(2, 1)));
        assert!(!board.uneaten_squares.contains(&(2, 2)));
    }

    #[test]
    fn test_winning_move() {
        let mut board = Board::new(3, 3);

        // Chomp at (0, 1) to force a winning move at (1, 0)
        board.chomp(0, 1);

        if let Some(winning_move) = board.winning_move() {
            assert_eq!(winning_move, (1, 0));
        } else {
            panic!("Expected a winning move, but none found.");
        }
    }
}
