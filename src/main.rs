use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use rand::Rng;

struct Game {
    field: [[char; 3]; 3],
    player_score: i32,
    robot_score: i32,
    current_player: char,
    cursor_x: usize,
    cursor_y: usize,
}

const cursor: &str = "▅";

impl Game {
    fn new() -> Game {
        Game {
            field: [[' '; 3]; 3],
            player_score: 0,
            robot_score: 0,
            current_player: 'X',
            cursor_x: 1,
            cursor_y: 1,
        }
    }

    fn print(&self) {
        println!("=====TermTacToe=====");
        println!("                Score");
        println!("      Player: {}  Robot: {}", self.player_score, self.robot_score);
        println!("   ###############");
        for (i, row) in self.field.iter().enumerate() {
            if i == self.cursor_y {
                print!("   # ");
                for (j, &cell) in row.iter().enumerate() {
                    if j == self.cursor_x {
                        print!("*");
                    } else {
                        print!("{}", cell);
                    }
                    print!(" | ");
                }
                println!("#");
            } else {
                println!("   #    {}    |    {}   |    {}    #", row[0], row[1], row[2]);
                println!("   #----------------------------#");
            }
        }
        println!("   ###############");
    }

    fn update(&mut self) {
        self.print();
        self.handle_input();
        if self.check_winner('X') {
            println!("Player wins!");
            self.player_score += 1;
            self.reset_game();
        } else if self.check_winner('O') {
            println!("Robot wins!");
            self.robot_score += 1;
            self.reset_game();
        }
    }

    fn handle_input(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "u" => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            "d" => {
                if self.cursor_y < 2 {
                    self.cursor_y += 1;
                }
            }
            "l" => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            "r" => {
                if self.cursor_x < 2 {
                    self.cursor_x += 1;
                }
            }
            "turn" => {
                if self.field[self.cursor_y][self.cursor_x] == ' ' {
                    self.field[self.cursor_y][self.cursor_x] = self.current_player;
                    self.switch_player();
                }
            }
            _ => (),
        }
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
        if self.current_player == 'O' {
            self.bot_move();
        }
    }

    fn bot_move(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..3);
            let y = rng.gen_range(0..3);
            if self.field[y][x] == ' ' {
                self.field[y][x] = 'O';
                break;
            }
        }
        self.switch_player();
    }

    fn check_winner(&self, player: char) -> bool {
        //rows
        for row in &self.field {
            if row.iter().all(|&cell| cell == player) {
                return true;
            }
        }
        //cols
        for i in 0..3 {
            if (0..3).all(|j| self.field[j][i] == player) {
                return true;
            }
        }
        //diagonals
        if (0..3).all(|i| self.field[i][i] == player) || (0..3).all(|i| self.field[i][2 - i] == player) {
            return true;
        }
        false
    }

    fn reset_game(&mut self) {
        self.field = [[' '; 3]; 3];
        self.current_player = 'X';
        self.cursor_x = 1;
        self.cursor_y = 1;
    }
}

fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        thread::sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }
}