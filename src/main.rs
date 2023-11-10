use chomp::Board;
use std::io;

fn main() {
    //Print welcome message
    println!("--------------------");
    println!();
    println!("CHOMP A.I.");
    println!();
    println!("--------------------");

    // Construct board
    let mut board: Board = board_setup();

    // Print initial board state
    println!("--------------------");
    board.print();
    println!("--------------------");

    // Start game loop
    play_game(&mut board);
}

/// Sets up and returns a new `Board` based on user input for the board size.
///
/// The function prompts the user to enter their desired board size (e.g., "3 5").
/// It validates the input, ensuring it consists of two space-separated numbers,
/// and creates a new `Board` with the specified width and height.
///
/// If the input is invalid, the function continues prompting the user until valid input is provided.
///
/// # Returns
///
/// Returns a new `Board` instance based on the user's input.
///
/// # Examples
///
/// ```
/// use chomp::Board;
///
/// let board = board_setup();
/// println!("Initialized board with size: {} x {}", board.width, board.height);
/// ```
fn board_setup() -> Board {
    let mut board_flag = false;
    let mut board: Board = Board::new(0, 0);
    while !board_flag {
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

        board = Board::new(num0,num1);
        board_flag = true;
    }
    board
}

/// Plays the Chomp game on the provided board, alternating between the human player and the AI.
///
/// The function prompts the human player for moves, validates the input, and updates the board accordingly.
/// After each human move, the AI determines its best move using the `winning_move` function or, if necessary,
/// chomps the furthest-right piece in the lowermost nonempty row to stall.
///
/// The game continues until there is only one uneaten square left, and the function announces the winner.
///
/// # Arguments
///
/// * `board` - A mutable reference to the Chomp game board.
///
/// # Examples
///
/// ```
/// use chomp::Board;
///
/// let mut board = Board::new(3, 3);
/// play_game(&mut board);
/// ```
fn play_game(board:&mut Board) {
    let mut previous_player: i32 = -1; // Track last player to tell who won. 1 is human player, -1 is A.I.

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

        // Check if it is a valid move
        if board.uneaten_squares.contains(&(num0, num1)) {
            // Player has eaten poisoned square. They lose.
            if num0 == 0 && num1 == 0 {
                board.chomp(num0, num1);

                println!("Your move:");
                board.print();
                println!("--------------------");

                continue;
            }

            // Chomp players move
            board.chomp(num0, num1);

            println!("Your move:");
            board.print();
            println!("--------------------");

            previous_player *= -1;

            // If one block left, it is the poison square, continue on to victory
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
