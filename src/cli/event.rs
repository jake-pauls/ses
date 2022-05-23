use anyhow::Result;
use ansi_term::Color::Blue;
use ansi_term::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use super::args::Args;

/// Performs a selection operation in a minial tui on a Vec of files
pub fn run_select_opt(args: &Args, opts: &mut Vec<&str>) -> Result<String> 
{
    let txt: String = "[📎] Pick a file to open in ".to_owned();

    // TODO: Build trait for output formatter
    let mut cmd: String = args.run.to_owned();
    cmd = Style::new().bold().paint(cmd).to_string();
    cmd = Blue.paint(cmd).to_string();

    let prompt: String = txt.clone() + &cmd;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&opts[..])
        .interact()
        .unwrap();

    let opt = opts.remove(selection);

    Ok(String::from(opt))
}