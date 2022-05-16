use clap::Parser;

#[derive(Parser)]
struct Wrapper {
    /// Traditional input for es
    input: String,
}

fn main() {
    let _args = Wrapper::parse();
}
