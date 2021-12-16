use atty::Stream;
use poetic::{interpreter::Interpreter, parser::Parser};
use std::{
    io::{self, Read},
    time::Instant,
};

fn main() {
    if atty::is(Stream::Stdin) {
        return;
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let run_now = Instant::now();

    let intermediate_now = Instant::now();
    let intermediate = Parser::parse_intermediate(&buf);
    println!(
        "parsing to intermediate took {}",
        intermediate_now.elapsed().as_secs_f64()
    );

    let instructions_now = Instant::now();
    let code = Parser::parse_instructions(&intermediate);
    println!(
        "parsing to instructions took {}",
        instructions_now.elapsed().as_secs_f64()
    );

    let mut interpreter = Interpreter::new(code);
    interpreter.run();

    println!("run took {}", run_now.elapsed().as_secs_f64());
}
