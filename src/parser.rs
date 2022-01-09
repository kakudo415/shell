pub enum Command {
    Empty,
    Simple {
        words: Vec<String>,
    },
    Pipe {
        upstream: Box<Command>,
        downstream: Box<Command>,
    },
}

pub fn parse(input: &String) -> Command {
    let words: Vec<String> = input.split_whitespace().map(|x| String::from(x)).collect();
    Command::Simple { words: words }
}
