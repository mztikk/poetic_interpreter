use atty::Stream;
use poetic::{interpreter::Interpreter, parser::Parser};
use std::io::{self, Read};

fn main() {
    if atty::is(Stream::Stdin) {
        return;
    }

    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let intermediate = Parser::parse_intermediate(&buf);
    let code = Parser::parse_instructions(&intermediate);
    let mut interpreter = Interpreter::new(code);
    interpreter.run();
}
