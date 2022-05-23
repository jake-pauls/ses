use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let args = cli::args::get_arg_list();

    let es = cli::process::spawn_es_process(&args);
    assert!(es.is_ok());
    
    // Retrieve and split file paths from es
    let output = String::from_utf8(es.unwrap().stdout).unwrap();
    let mut files: Vec<&str> = output.split("\n").collect();

    // Run selection option for multiple files
    let target = cli::event::run_select_opt(&args, &mut files).unwrap();

    // Spawn cmd to do something, a default is always set
    let cmd = cli::process::spawn_run_process(&args, &target);
    assert!(cmd.is_ok());

    Ok(())
}
