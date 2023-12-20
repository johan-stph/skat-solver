use std::io::BufRead;
use skat_solver::{Game, Input, Output};


struct ConsoleInput {
    stdin: std::io::Stdin,
}

struct ConsoleOutput {}
impl Output for ConsoleOutput {
    fn display(&self, message: &str) {
        println!("{}", message);
    }
}

impl Input for ConsoleInput {
    fn get_input(&self) -> String {
        let mut line = String::new();
        self.stdin.lock().read_line(&mut line).unwrap();
        line
    }
}



fn main() {

    // create game
    // start game
    // parse input
    // if valid execute command
    // display result of command

    let game = Game::new(Box::new(ConsoleInput {
        stdin: std::io::stdin(),
    }), Box::new(ConsoleOutput {}));

}
