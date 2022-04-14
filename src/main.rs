use atty::Stream;
use clap::Parser;
use poetic::{
    interpreter::Interpreter,
    optimizer::{Optimize, Optimizer},
};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    time::Instant,
};

#[derive(Parser, Debug)]
struct Cli {
    /// Input file
    #[clap(parse(from_os_str), help = "Input file to interpret and run")]
    input: Option<PathBuf>,

    #[clap(
        short,
        long,
        requires("input"),
        help = "Don't read code from stdin but instead interpret it as input if IN instructions are called. Requires input."
    )]
    no_stream: bool,

    #[clap(short, long, help = "Prints the time it took to parse and run")]
    time: bool,

    #[clap(
        short,
        long,
        help = "Size of fixed memory, if ommitted dynamic memory is used"
    )]
    memory_size: Option<usize>,

    #[clap(short, long, help = "Disables optimizations")]
    disable_optimizations: bool,
}

fn main() {
    // if atty::is(Stream::Stdin) {
    //     return;
    // }

    let cli = Cli::parse();

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
    let intermediate = poetic::parser::Parser::parse_intermediate(&buf);
    if cli.time {
        println!(
            "parsing to intermediate took {}",
            intermediate_time.elapsed().as_secs_f64()
        );
    }

    let instructions_time = Instant::now();
    let mut code = poetic::parser::Parser::parse_instructions(&intermediate);
    if cli.time {
        println!(
            "parsing to instructions took {}",
            instructions_time.elapsed().as_secs_f64()
        );
    }

    // let out_file = Arc::new(Mutex::new(File::create("output.txt").unwrap()));

    if !cli.disable_optimizations {
        let optimize_time = Instant::now();
        let optimizer = Optimizer;
        code = optimizer.optimize(&code);
        if cli.time {
            println!("optimizing took {}", optimize_time.elapsed().as_secs_f64());
        }
    }

    match cli.memory_size {
        Some(size) => {
            let mut interpreter = Interpreter::new(code).with_fixed_size_memory(size);
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
