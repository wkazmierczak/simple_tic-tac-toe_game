use std::io;
 
fn main() {

    let mut board = [[' '; 3]; 3];
    board_printer(&mut board);

    let mut row = 0;
    let mut col = 0;

    for i in 0..100 {
        let mut user_input = String::new();
        let mut cmd : char; // place for the command

        let player = i%2+1;

        println!("Type your command player {player}:");
        let _ = io::stdin().read_line(&mut user_input); // get string from the user input
        cmd = user_input.chars().nth(0).unwrap(); // get the first char from the given string

        let int_digit: usize = cmd as usize - '0' as usize;

        if 0 < int_digit && int_digit <= 3 {
            row = 0;
            col = int_digit-1;
        }
        if 3 < int_digit && int_digit <= 6 {
            row = 1;
            col = int_digit-4;
        }
        if 6 < int_digit && int_digit <= 9 {
            row = 2;
            col = int_digit-7;
        }
        if 0 >= int_digit || int_digit > 9{
            println!("bad_input");
            break;
        }

        update_board(&mut board, row, col, player);

        board_printer(&mut board);

        match check_winner(&board) {
            'X' => {
                println!("Player 1 wins!");
                break;
            },
            'O' => {
                println!("Player 2 wins!");
                break;
            },
            '-' => {
                println!("Draw!");
                break;
            },
            ' ' => println!("No winner yet"),
            _ => println!("Something went wrong!")
        }
        
    }
}

fn update_board(board : &mut [[char; 3]; 3], row: usize, col: usize, player: u32) {
    if board[row][col] == ' ' {
        if player == 1 {
            board[row][col] = 'X';
        } else {
            board[row][col] = 'O';
        }
    } else {
        println!("Cell is already occupied");
    }
}

fn check_winner(board: &[[char; 3]; 3]) -> char {
    for i in 0..3 {
        if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return board[i][0];
        }
        if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return board[0][i];
        }
    }

    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[0][0];
    }
    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[0][2];
    }
    let mut flag = 1;
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == ' ' {
                flag = 0;
                break;
            }
        }
        if flag == 0 {
            break;
        }
    }
    if flag == 1 {
        return '-';
    }
    ' '
}

fn board_printer (board : &mut [[char; 3]; 3]) {
    println!("{:?}", board[0]);
    println!("{:?}", board[1]);
    println!("{:?}", board[2]);
}
