use anyhow::Result;

mod cli;
use cli::{args, event, log, process};
use cli::log::LogType;

fn main() -> Result<()> {
    let args = args::get_arg_list();

    let es = process::spawn_es_process(&args);
    assert!(es.is_ok());

    let results = es.unwrap().stdout; 
    if results.is_empty() {
        log::base("No results found!", LogType::Error);
        return Ok(());
    }

    // Retrieve and split file paths from es, last element is always an new line
    // results are a vector of u8s, ie: unicode from the es output
    let output = String::from_utf8(results).unwrap();
    let mut files: Vec<&str> = output.split("\n").collect();
    files.pop();

    // Run selection option for multiple files
    let target = event::run_select_opt(&args, &mut files).unwrap();

    // Spawn cmd to do something, a default is always set
    let cmd = process::spawn_run_process(&args, &target);
    assert!(cmd.is_ok());

    Ok(())
}
