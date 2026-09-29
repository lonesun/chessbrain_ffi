fn main() -> std::io::Result<()> {
    let mut stockfish_process = std::process::Command::new("../Stockfish/src/stockfish")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
}
