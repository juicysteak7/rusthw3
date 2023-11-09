use chomp::Board;
use std::io;

fn main() {
    println!("--------------------");
    println!();
    println!("CHOMP A.I.");
    println!();
    println!("--------------------");

    let mut board_setup = false;
    let mut board: Board = Board::new(0, 0);
    while !board_setup {
        // Create a mutable string to store the user's input
        let mut input = String::new();

        // Print a prompt to the user
        println!("Please enter your desired board size (Example: 3 5)");

        // Read user input and store it in the 'input' string
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Split the input into words and collect them into a Vec
        let words: Vec<&str> = input.split_whitespace().collect();

        // Ensure we have exactly two words
        if words.len() != 2 {
            println!("Please enter exactly two numbers separated by a space.");
            continue;
        }

        // Parse the words into usize variables
        let num0: usize = match words[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the first number.");
                continue;
            }
        };

        let num1: usize = match words[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the second number.");
                continue;
            }
        };

        board = Board::new(num0, num1);
        board_setup = true;
    }

    println!("--------------------");
    board.print();
    println!("--------------------");

    let mut previous_player: i32 = -1;

    while board.uneaten_squares.len() > 1 {
        // Create a mutable string to store the user's input
        let mut input = String::new();

        // Print a prompt to the user
        println!("Please enter your move (Example: 2 2)");

        // Read user input and store it in the 'input' string
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Split the input into words and collect them into a Vec
        let words: Vec<&str> = input.split_whitespace().collect();

        // Ensure we have exactly two words
        if words.len() != 2 {
            println!("Please enter exactly two numbers separated by a space.");
            continue;
        }

        // Parse the words into usize variables
        let num0: usize = match words[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the first number.");
                continue;
            }
        };

        let num1: usize = match words[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input for the second number.");
                continue;
            }
        };

        if num0 >= board.width || num1 >= board.height {
            println!("Please use numbers smaller than inital width and height");
            continue;
        }

        if board.uneaten_squares.contains(&(num0, num1)) {
            // Chomp players move
            board.chomp(num0, num1);

            board.print();
            println!("--------------------");

            previous_player *= -1;

            if board.uneaten_squares.len() <= 1 {
                continue;
            }
        } else {
            println!("Please only chomp uneaten squares.");
            continue;
        }

        // Determine best A.I. move then Chomp
        if let Some(result) = board.winning_move() {
            board.chomp(result.0, result.1);
        } else {
            'outer: for i in (0..board.width).rev() {
                for j in (0..board.height).rev() {
                    if board.uneaten_squares.contains(&(i, j)) {
                        board.chomp(i, j);
                        break 'outer;
                    }
                }
            }
        }

        println!("A.I.'s move:");
        board.print();
        println!("--------------------");

        previous_player *= -1;
    }

    if previous_player == 1 {
        println!("You win!")
    } else {
        println!("You lose!")
    }
}
