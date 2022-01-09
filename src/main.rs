mod parser;
mod executor;

use std::io::{stdin, stdout, Write};

fn main() {
    main_loop();
}

fn main_loop() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("ERROR: read_line()");

        let input = parser::parse(&input);
        executor::execute(input);
    }
}
