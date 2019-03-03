use std::io::{self, BufRead, Write};

fn read(s: &str) -> &str { s }

fn eval(s: &str) -> &str { s }

fn print(s: &str) -> &str { s }

fn rep(s: &str) -> &str {
    print(eval(read(s)))
}

const PROMPT: &str = "user> ";

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input_iter = stdin.lock().lines();
    loop {
        print!("{}", PROMPT);
        stdout.flush().expect("WARNING: Flushing standard output failed");
        let line = &input_iter.next().unwrap().unwrap();
        println!("{}", rep(line))
    }
}
