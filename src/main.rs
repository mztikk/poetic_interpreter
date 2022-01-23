use atty::Stream;
use poetic::{
    interpreter::{default_input_stream, default_output_stream, Interpreter},
    optimizer::Optimizer,
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

    #[structopt(
        short,
        long,
        help = "Size of fixed memory, if ommitted dynamic memory is used"
    )]
    memory_size: Option<usize>,

    #[structopt(short, long, help = "Disables optimizations")]
    disable_optimizations: bool,
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
    let mut code = Parser::parse_instructions(&intermediate);
    if cli.time {
        println!(
            "parsing to instructions took {}",
            instructions_time.elapsed().as_secs_f64()
        );
    }

    // let out_file = Arc::new(Mutex::new(File::create("output.txt").unwrap()));

    if !cli.disable_optimizations {
        let optimize_time = Instant::now();
        let mut optimizer = Optimizer::new(code);
        code = optimizer.optimize();
        if cli.time {
            println!("optimizing took {}", optimize_time.elapsed().as_secs_f64());
        }
    }

    match cli.memory_size {
        Some(size) => {
            let mut interpreter = Interpreter::new_fixed_size(code, size);
            interpreter.run();
        }
        None => {
            let mut interpreter = Interpreter::new(code);
            interpreter.run();
        }
    }

    if cli.time {
        println!("run took {}", run_time.elapsed().as_secs_f64());
    }
}
