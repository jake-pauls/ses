use ansi_term::Color::{Cyan, Fixed};
use anyhow::Result;
use std::process::{Command, Output};

use super::args::Args;
use super::log;
use super::log::LogType;

/// Spawns 'es' process with provided arguments
pub fn spawn_es_process(args: &Args) -> Result<Output> {
    let whole_words = if args.whole_words_flag { "-whole-words" } else { "" };
    let case = if args.case_flag { "-case" } else { "" };
    let path = if args.path_flag { "-match-path" } else { "" };
    let sort = if args.sort_flag { "-s" } else { "" };

    let offset: u32 = if args.offset.is_none() { 0 } else { args.offset.unwrap() };

    let mut es_cmd = cmd("cmd");
    es_cmd
        .args(&["/C", "es"])
        .args(&[&args.file, whole_words, case, path, sort])
        .args(&["-offset", &offset.to_string()]);

    // Check if a regex was included
    if !args.regex.is_none() {
        es_cmd.args(&["-regex", &args.regex.as_ref().unwrap()]);
    }

    // Check if a value for max was included
    if !args.max.is_none() {
        es_cmd.args(&["-max-results", &args.max.unwrap().to_string()]);
    }

    // Start es process and return output
    let es_out = es_cmd
        .output()
        .expect("es command failed to start, do you have es installed and on your system PATH?");

    Ok(es_out)
}

/// Spawns a run process if one was passed in the CLI
pub fn spawn_run_process(args: &Args, file: &str) -> Result<()> {
    let mut run_cmd: Command;

    if args.run.eq("explorer") {
        let path = get_explorer_path(file);
        run_cmd = cmd(&args.run);
        run_cmd
            .arg(path)
            .status()
            .expect("explorer failed to start, is something broken?");

        let msg = Fixed(15).paint("Successfully opened directory ").to_string();
        let out = Cyan.paint(get_explorer_path(file)).to_string();
        log::base(&(msg + &out), LogType::Success);
    } else {
        run_cmd = cmd("cmd");
        run_cmd.args(&["/C", &args.run]).arg(file).status().expect(
            "command failed to start, do you have the command installed and on your system PATH?",
        );
    }

    Ok(())
}

/// Creates a new command with 'cmd' (Windows-isms)
fn cmd(cmd: &str) -> Command {
    Command::new(cmd)
}

/// Convert a file directory to a path that can be opened in File Explorer
/// ie: C:\ses\src\main.rs -> C:\ses\src\
fn get_explorer_path(win_path: &str) -> String {
    let (dir, _) = win_path.rsplit_once('\\').unwrap();

    String::from(dir)
}
