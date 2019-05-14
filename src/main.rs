extern crate rand;
use std::io;


use rand::{thread_rng, Rng};
struct User {
    score: u64,
    marker: String,
    games: u64,
    draws: u64
}


impl User {
    pub fn print_score(&self) -> () {
        println!("Your score is {} out of {}, with {} draws", self.score, self.games, self.draws)
    }

    pub fn opposite_marker(&self) -> String {
        if self.marker == "X"{
            return "O".to_string()
        }
        return "X".to_string()

    }
}

struct Board {
    state: [[String;3];3]
}

impl Board {
    pub fn new() -> Board{
        Board{state:[["-".to_string(), "-".to_string(), "-".to_string()],
                     ["-".to_string(), "-".to_string(), "-".to_string()],
                     ["-".to_string(), "-".to_string(), "-".to_string()]]
             }
    }

    pub fn display_board(&self) -> () {
        self.state.into_iter().for_each(|el| println!("{:?}", el));
    }

    pub fn make_move(&mut self,x: usize, y: usize, marker: &str) -> () {
        if !((4 > x) && (x > 0) && (4 > y) && (y > 0)){
            panic!("Invalid Input")
        }
        if self.state[x-1][y-1] != "-" {
            panic!("Square occupied!")
        }
        self.state[x-1][y-1] = marker.to_string()
    }

    pub fn evaluate_game_state(&self) -> String {

        let x_win = ["X","X","X"];
        let o_win = ["O","O","O"];

        let x_return = "X".to_string();
        let o_return = "O".to_string();
        //check all the rows
        for row in self.state.into_iter() {
            if(row == &x_win){
                return x_return
            }
            if(row == &o_win){
                return o_return
            }
        }

        //check the diagonals

        let diag1 = [&self.state[0][0], &self.state[1][1], &self.state[2][2]];
        let diag2 = [&self.state[0][2], &self.state[1][1], &self.state[2][0]];

        if diag1 == x_win || diag2 == x_win{
            return x_return
        }

        if diag1 == o_win || diag2 == o_win{
            return o_return
        }


        //check the columns
        let col1 = [&self.state[0][0], &self.state[1][0], &self.state[2][0]];
        let col2 = [&self.state[0][1], &self.state[1][1], &self.state[2][1]];
        let col3 = [&self.state[0][2], &self.state[1][2], &self.state[2][2]];

         if col1 == x_win || col2 == x_win || col3 == x_win {
            return x_return
        }

        if col1 == o_win || col2 == o_win || col3 == o_win{
            return o_return
        }

        if !self.state.into_iter().any(|row| row.into_iter().any(|cell| cell == "-")){
            return "D".to_string()
        }

        return "-".to_string()
    }
}


fn main() {
    let mut user = User{
        score: 0,
        marker: get_x_or_o(),
        games: 0,
        draws: 0
    };
    play_loop(&mut user);

}

fn play_loop(user: &mut User) {
    let mut playing = initiate_game_loop();
    while playing {
        user.games += 1;
        run_game(user);
        user.print_score();
        playing = initiate_game_loop();
    }
    user.print_score();
    println!("Thank you for playing!");
}

fn run_game(user: &mut User){
    println!("{}", &user.marker);
    let mut board = Board::new();
    while true{
        board.display_board();
        println!("Make a move for {}",  &user.marker);
        let coords = get_user_coordinates(&board);
        board.make_move(coords.0,coords.1, &user.marker);
        board.display_board();
        if board.evaluate_game_state() != "-" {
            break
        }
        let coords2 = ai_generate_move(&board);
        board.make_move(coords2.0,coords2.1, &user.opposite_marker());
        if board.evaluate_game_state() != "-" {
            break
        }
    }
    board.display_board();
    if &board.evaluate_game_state() == &user.marker {
        println!("You won!");
        user.score += 1
    } else if  &board.evaluate_game_state() == "D"  {
        println!("You drew!");
        user.draws += 1
    } else {
        println!("You lost!");
    }

}

fn ai_generate_move(board: &Board) -> (usize, usize) {
    let mut array_of_moves = Vec::new();
    for (i, row) in board.state.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if(cell == "-"){
                array_of_moves.push((i+1, j+1));
            }
        }
    }
    thread_rng().shuffle(&mut array_of_moves);
    return array_of_moves[0].clone()
}

fn get_user_coordinates(board: &Board) -> (usize, usize) {
    println!("Please select a row");
    let mut x =String::new();
    let stdin = io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("Please select a column");
    let mut y =String::new();
    let stdin = io::stdin().read_line(&mut y).expect("Failed to read line");

    let y: usize = y.trim().parse().unwrap();
    let x: usize = x.trim().parse().unwrap();

    if !((4 > x) && (x > 0) && (4 > y) && (y > 0)){
        println!("Coordinates were invalid. Please try again");
        return get_user_coordinates(board);
    }
    if board.state[x-1][y-1] != "-" {
        println!("Square occupied!");
        return get_user_coordinates(board);
    }
    return (x, y)

}
fn initiate_game_loop() -> bool {
    println!("Would you like to play? Y/N");
    let mut choice=String::new();
    let mut play_again = true;
    let stdin = io::stdin().read_line(&mut choice)
    .expect("Failed to read line");
    match choice.trim().as_ref() {
        "Y" | "yes" | "Yes" | "y" => play_again = true,
        "quit" | "q" | "exit" => panic!("User quit"),
        _ => play_again = false
    }
    return play_again;

}

fn get_x_or_o() -> String{
    println!("Welcome to tic-tac-toe");
    println!("X's or O's?");
    let mut character=String::new();
    let mut repeat = true;
    while repeat {
        let stdin = io::stdin().read_line(&mut character)
        .expect("Failed to read line");;
        match character.trim().as_ref() {
            "X" | "O" => repeat = false,
            "quit" | "q" | "exit" => panic!("User quit"),
            _ => println!("{} is not a valid input", character.trim())
        }
        if repeat{
            character = "".to_string()
        }
    }
    return character.trim().into();
}
