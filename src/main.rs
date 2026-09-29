fn main() -> std::io::Result<()> {
    let mut stockfish_process = std::process::Command::new("../Stockfish/src/stockfish")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let result = (|| -> std::io::Result<()> {
        let mut input = stockfish_process.stdin.take().expect("Failed to take stdin");
        let mut output = std::io::BufReader::new(stockfish_process.stdout.take().expect("Failed to take stdout"));

        send(&mut input, "position startpos moves e2e4")?;
        send(&mut input, "go movetime 1000")?;
    })();

    stockfish_process.wait()?;
    result
}
