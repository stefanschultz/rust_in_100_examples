use std::io::{self, Write};  // Import I/O traits, including Write for flushing stdout

fn main() {
    // Initialize the 3×3 board as an array of 9 chars, all starting as spaces.
    let mut board: [char; 9] = [' '; 9];

    // 'X' always starts first.
    let mut current_player: char = 'X';

    // Main game loop: continues until someone wins or it's a draw.
    loop {
        // Print the current board state.
        print_board(&board);

        // Prompt the current player for their move.
        // We use `print!` so that we can flush and keep the cursor on the same line.
        print!("Player {}, enter a move (1-9): ", current_player);
        io::stdout().flush().expect("Failed to flush stdout");

        // Read the user’s input line (including the newline).
        let input = read_line();

        // Try to parse the trimmed input into a usize. If it’s not between 1 and 9, reject.
        let idx: usize = match input.trim().parse::<usize>() {
            Ok(num) if (1..=9).contains(&num) => num - 1,  // Convert 1–9 to 0–8
            _ => {
                println!("Invalid input. Please enter a number from 1 to 9.");
                continue;  // Restart the loop without changing the board or player
            }
        };

        // If the chosen cell is not empty, inform the player and retry.
        if board[idx] != ' ' {
            println!("Cell {} is already occupied. Try again.", idx + 1);
            continue;
        }

        // Place the current player's mark on the board.
        board[idx] = current_player;

        // After the move, check if the current player has won.
        if check_win(&board, current_player) {
            print_board(&board);
            println!("Player {} wins!", current_player);
            break;  // Exit the loop, ending the game
        }

        // If all cells are filled and nobody has won, it's a draw.
        if board.iter().all(|&c| c != ' ') {
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        // Switch players: X → O, or O → X.
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

/// Prints the current board in a nice 3×3 grid.
/// The board slice has 9 elements, indexed 0 through 8.
fn print_board(board: &[char; 9]) {
    println!();  // Blank line for spacing
    for row in 0..3 {
        // Print a single row: cells at indices row*3, row*3+1, row*3+2
        println!(" {} | {} | {} ",
            board[row * 3],
            board[row * 3 + 1],
            board[row * 3 + 2]
        );
        // Between rows, print a separator; after the last row we skip this.
        if row < 2 {
            println!("---+---+---");
        }
    }
    println!();  // Blank line after the board
}

/// Checks whether the given `player` (either 'X' or 'O') has any winning line.
/// Returns true if any of the 8 win conditions is fully occupied by `player`.
fn check_win(board: &[char; 9], player: char) -> bool {
    // All possible ways to win: 3 rows, 3 columns, 2 diagonals.
    const WIN_CONDITIONS: [[usize; 3]; 8] = [
        [0, 1, 2],  // Top row
        [3, 4, 5],  // Middle row
        [6, 7, 8],  // Bottom row
        [0, 3, 6],  // Left column
        [1, 4, 7],  // Middle column
        [2, 5, 8],  // Right column
        [0, 4, 8],  // Main diagonal
        [2, 4, 6],  // Anti-diagonal
    ];

    // Iterate through each win condition; check if all 3 spots match `player`.
    WIN_CONDITIONS.iter().any(|&line| {
        line.iter().all(|&i| board[i] == player)
    })
}

/// Reads a full line from standard input, returning it as a String.
/// If reading fails, the program will panic with an error message.
fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
