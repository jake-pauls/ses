use clap::Parser;

/// Find a file (and actually do something with it) using es 🦸📎
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Target file
    pub file: String,
    /// Run this command on the selected file
    #[clap(short, default_value = "explorer")]
    pub run: String,
    /// Match case when searching indexed files
    #[clap(short)]
    pub case: bool,
    /// Match whole words when searching indexed files
    #[clap(short)]
    pub whole_words: bool,
}

pub fn get_arg_list() -> Args {
    Args::parse()
}
