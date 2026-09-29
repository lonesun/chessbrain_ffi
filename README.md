## Overview

This crate provides a Rust interface for communicating with the Stockfish chess engine. It uses the **Universal Chess Interface (UCI) protocol** to do so, and includes functions to send commands to Stockfish and receive responses.

## Usage

`chessbrain_ffi` is intended to be used as a library in Rust projects that need to interact with the Stockfish chess engine. For the time being, you can witness it in action, but you will need to [manually download](https://stockfishchess.org/download/) the Stockfish executable and include it in the `Stockfish/` folder.

```rust
fn main() -> std::io::Result<()> {
    let mut stockfish_process = std::process::Command::new("Stockfish/stockfish-windows-x86-64-universal.exe")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    // ... code omitted for brevity
    Ok(())
}
```

This is basically fragile of course, since not everyone is running Windows. The above path assumes a Windows environment (since my sketchdev setup is on Windows) and will need to be adjusted for other operating systems. You can just change the path to whatever you want, but I can't currently guarantee that it will work correctly on non-Windows systems. The above snippet can be found in `main.rs`.
