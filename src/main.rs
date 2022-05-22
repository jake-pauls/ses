use std::process::Command;
use std::io::{self, Write};
use anyhow::Result;
use clap::Parser;

/// Find a file (and actually do something with it) using es
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Target file
    file: String,
    /// Run this command on the es output 
    #[clap(short)]
    run: String,
    /// Match case when searching indexed files
    #[clap(short)]
    case: bool,
    /// Match whole words when searching indexed files
    #[clap(short)]
    whole_words: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut ww = String::from(""); 
    if args.whole_words {
        ww = String::from("-w");
    }

    let mut c: String = String::from("");
    if args.case {
        c = String::from("-i");
    }

    let mut es_cmd = Command::new("cmd");
    es_cmd.args(&["/C", "es"])
          .arg(&args.file)
          .arg(ww)
          .arg(c);

    // Start es process and write to stdout
    let es_out = es_cmd.output().expect("es command failed to start, do you have es installed and on your system PATH?");
    io::stdout().write_all(&es_out.stdout).unwrap();

    // Retrieve file paths from es
    // Parse multiple file paths ???
    let res = String::from_utf8(es_out.stdout).unwrap();

    // Create cmd to do something
    let mut run_cmd = Command::new("cmd");
    run_cmd.args(&["/C", &args.run]).arg(res);
    run_cmd.status().expect("command failed to start, do you have the command installed and on your system PATH?");

    Ok(())
}
