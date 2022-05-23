use clap::Parser;
 
/// Find a file (and actually do something with it) using es 🦸📎 
#[derive(Parser)]
#[clap(author, version, about, usage = "ses <FILE> [OPTIONS]")]
pub struct Args {
    /// Target file
    pub file: String,
    /// Run this command on the es output 
    #[clap(short)]
    pub run: Option<String>,
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
