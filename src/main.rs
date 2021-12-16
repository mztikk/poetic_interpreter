use atty::Stream;
use poetic::{interpreter::Interpreter, parser::Parser};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    time::Instant,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Input file
    #[structopt(parse(from_os_str), required_if("no_stream", "true"))]
    input: Option<PathBuf>,

    #[structopt(short, long)]
    no_stream: bool,
}

fn main() {
    // if atty::is(Stream::Stdin) {
    //     return;
    // }

    let cli = Cli::from_args();

    let mut buf: String;
    if atty::is(Stream::Stdin) || cli.no_stream {
        buf = fs::read_to_string(&cli.input.expect("Input file required"))
            .expect("Failed to read file");
    } else {
        buf = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut buf).unwrap();
    }

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
