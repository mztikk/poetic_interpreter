use atty::Stream;
use poetic::{
    interpreter::{default_input_stream, default_output_stream, Interpreter},
    parser::Parser,
};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Instant,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Input file
    #[structopt(parse(from_os_str), help = "Input file to interpret and run")]
    input: Option<PathBuf>,

    #[structopt(
        short,
        long,
        requires("input"),
        help = "Don't read code from stdin but instead interpret it as input if IN instructions are called. Requires input."
    )]
    no_stream: bool,

    #[structopt(short, long, help = "Prints the time it took to parse and run")]
    time: bool,
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

    let run_time = Instant::now();

    let intermediate_time = Instant::now();
    let intermediate = Parser::parse_intermediate(&buf);
    if cli.time {
        println!(
            "parsing to intermediate took {}",
            intermediate_time.elapsed().as_secs_f64()
        );
    }

    let instructions_time = Instant::now();
    let code = Parser::parse_instructions(&intermediate);
    if cli.time {
        println!(
            "parsing to instructions took {}",
            instructions_time.elapsed().as_secs_f64()
        );
    }

    // let out_file = Arc::new(Mutex::new(File::create("output.txt").unwrap()));

    let mut interpreter = Interpreter::new(code);
    // let mut interpreter = Interpreter::new_io(
    //     code,
    //     Box::new(default_input_stream),
    //     Box::new(move |s| {
    //         out_file.lock().unwrap().write_all(s.as_bytes()).unwrap();
    //     }),
    //     // Box::new(default_output_stream),
    // );
    interpreter.run();

    if cli.time {
        println!("run took {}", run_time.elapsed().as_secs_f64());
    }
}
