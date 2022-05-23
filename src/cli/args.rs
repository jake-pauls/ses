use clap::Parser;

/// Find a file (and actually do something with it) using es 🦸📎
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Target file
    pub file: String,
    /// Run this command on the selected file
    #[clap(short, long = "run", default_value = "explorer")]
    pub run: String,
    /// Compare results against a regex expression, escape spaces with double quotes
    #[clap(long = "regex")]
    pub regex: Option<String>,
    /// Match case when searching indexed files
    #[clap(short, long = "case")]
    pub case_flag: bool,
    /// Match whole words when searching indexed files
    #[clap(short, long = "whole-words")]
    pub whole_words_flag: bool,
    /// Match full path and filename
    #[clap(short, long = "match-path")]
    pub path_flag: bool,
    /// Show results from the zero-based <OFFSET> onwards
    #[clap(short, long = "offset")]
    pub offset: Option<u32>,
    /// Limit the number of results to <NUM>
    #[clap(short, long = "max-results")]
    pub max: Option<u32>,
    /// Sort by full path
    #[clap(short, long = "sort")]
    pub sort_flag: bool,
}

pub fn get_arg_list() -> Args {
    Args::parse()
}
