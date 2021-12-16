use atty::Stream;
use poetic::{Interpreter, Parser};
use std::io::{self, Read};

fn main() {
    if atty::is(Stream::Stdin) {
        return;
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut interpreter = Interpreter::new();
    let mut parser = Parser::new(buf);
    for instruction in parser.code {
        interpreter.execute(&instruction);
    }
}
