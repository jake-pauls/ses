use std::io::{self, Write};
use std::process::{Command, Output};
use anyhow::Result;
use super::args::Args;

/// Spawns 'es' process with provided arguments
pub fn spawn_es_process(args: &Args) -> Result<Output> {
    // Check boolean flags
    // TODO: add more robust checks for es flags?
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

    // Start es process and return output 
    let es_out = es_cmd.output().expect("es command failed to start, do you have es installed and on your system PATH?");

    Ok(es_out)
}

/// Spawns a run process if one was passed in the CLI
pub fn spawn_run_process(args: &Args, file: &str) -> Result<()> {
    let mut run_cmd = Command::new("cmd");
    run_cmd.args(&["/C", &args.run.as_ref().unwrap()]).arg(file);
    run_cmd.status().expect("command failed to start, do you have the command installed and on your system PATH?");

    Ok(())
}