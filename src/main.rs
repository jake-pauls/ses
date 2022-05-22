use std::process::Command;
use std::io::{self, Write};
use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let args = cli::args::get_arg_list();

    // Check boolean flags
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
    let file_path = String::from_utf8(es_out.stdout).unwrap();

    // Create cmd to do something
    if !args.run.is_none() {
        let mut run_cmd = Command::new("cmd");
        run_cmd.args(&["/C", &args.run.unwrap()]).arg(file_path);
        run_cmd.status().expect("command failed to start, do you have the command installed and on your system PATH?");
    }

    Ok(())
}
