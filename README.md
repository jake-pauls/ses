<h3 align="center">⚡📎</h3>

### Why?
While working, my mentor and I were sick of copying the `es` output from the terminal - we actually wanted to do something with it. 

Also, piping from Powershell (or even WSL output) isn't fun and a return result of multiple files proves to be a pain.

### Build Local Executable

```
cargo build --target x86_64-pc-windows-gnu
```

The executable will be placed in `target/x86_64-pc-windows-gnu/debug/ses.exe`