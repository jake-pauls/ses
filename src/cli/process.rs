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

    let mut es_cmd = cmd("cmd");
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
    let mut run_cmd: Command;

    if args.run.eq("explorer") {
        let path = get_explorer_path(file);
        run_cmd = cmd(&args.run);
        run_cmd.arg(path)
            .status()
            .expect("explorer failed to start, is something broken?");
    } else {
        run_cmd = cmd("cmd");
        run_cmd.args(&["/C", &args.run])
            .arg(file)
            .status()
            .expect("command failed to start, do you have the command installed and on your system PATH?");
    }

    Ok(())
}

/// Creates a new command with 'cmd' (Windows-isms)
fn cmd(cmd: &str) -> Command {
    Command::new(cmd)
}

/// Convert a file directory to a path that can be opened in File Explorer
/// By default, Windows will open a file with a handle in it's default editor
/// As such, this attempts to provide control to ses users
/// ie: 'main.rs' would open in 'code'
fn get_explorer_path(win_path: &str) -> String {
    let (dir, _) = win_path.rsplit_once('\\').unwrap();

    String::from(dir)
}