<h3 align="center">🦸📎</h3>

### Why?
While working, my mentor and I were sick of copying the `es` output from the terminal. 

Also, piping from Powershell (or even WSL output) isn't fun and handling a return result of multiple files proves to be a pain.

<<<<<<< HEAD
Everything is created by [Voidtools](https://www.voidtools.com/), not me. Get `es` [here](https://www.voidtools.com/support/everything/command_line_interface/).
=======
*Everything* (search engine) is created by [Voidtools](https://www.voidtools.com/), not me. Get `es` [here](https://www.voidtools.com/support/everything/command_line_interface/).
>>>>>>> main

### Options

```
ses 0.1.0
Find a file (and actually do something with it) using es 🦸📎

USAGE:
    ses.exe [OPTIONS] <FILE>

ARGS:
    <FILE>    Target file

OPTIONS:
    -c, --case                 Match case when searching indexed files
    -h, --help                 Print help information
    -m, --max-results <MAX>    Limit the number of results to <NUM>
    -o, --offset <OFFSET>      Show results from the zero-based <OFFSET> onwards
    -p, --match-path           Match full path and filename
    -r, --run <RUN>            Run this command on the selected file [default: explorer]
        --regex <REGEX>        Compare results against a regex expression, escape spaces with double
                               quotes
    -s, --sort                 Sort by full path
    -V, --version              Print version information
    -w, --whole-words          Match whole words when searching indexed files
```

### Crates
- [anyhow](https://crates.io/crates/anyhow)
- [ansi_term](https://crates.io/crates/ansi_term) 
- [clap](https://crates.io/crates/clap)
- [dialoguer](https://crates.io/crates/dialoguer) 

### Build Local Executable

```
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be placed in `target/x86_64-pc-windows-gnu/release/ses.exe`