use std::process::Command;
use std::io::{self, Write};
use clap::Parser;

/// find a file (and do something with it) using es
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Target file
    file: String,
    /// Match case
    #[clap(short)]
    case: bool,
}

fn main() {
    let args = Args::parse();
    println!("input: {}", &args.file);
    println!("case: {}", &args.case);

    let mut es_cmd = Command::new("es");
    es_cmd.arg(&args.file);

    let es_out = es_cmd.output().expect("es command failed to start, do you have es installed and on your system PATH?");
    io::stdout().write_all(&es_out.stdout).unwrap();

    println!("process exited with code {}", es_out.status);
}
