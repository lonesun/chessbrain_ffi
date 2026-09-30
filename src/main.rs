use chessbrain_ffi::{Engine};

fn main() -> std::io::Result<()> {
    let mut engine = Engine::start("Stockfish/stockfish-windows-x86-64-universal.exe")?;

    let result = (|| -> std::io::Result<()> {
        let mut input = engine.stockfish_process.stdin.as_mut().expect("Failed to take stdin");
        let mut output = std::io::BufReader::new(engine.stockfish_process.stdout.take().expect("Failed to take stdout"));

        /* Initialise communication with Stockfish */
        send(&mut input, "uci")?; // UCI = Universal Chess Interface
        receive(&mut output, "uciok")?; // Acknowledgement from Stockfish
        send(&mut input, "isready")?; // Check if Stockfish is ready
        receive(&mut output, "readyok")?; // Readiness confirmation

        send(&mut input, "position startpos moves e2e4")?;
        send(&mut input, "go movetime 1000")?;
        let reply = receive(&mut output, "bestmove")?;
        let best_move = reply.split_whitespace().nth(1).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse best move")
        });
        println!("Black's reply: {}", best_move.unwrap_or("none"));
        send(&mut input, "quit")?;
        Ok(())
    })();

    if result.is_err() {
        let _ = engine.stockfish_process.kill();
    }
    engine.stockfish_process.wait()?;
    result
}
