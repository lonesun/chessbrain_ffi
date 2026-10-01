use chessbrain_ffi::{Engine};

fn main() -> std::io::Result<()> {
    let mut engine = Engine::start("Stockfish/stockfish-windows-x86-64-universal.exe")?;

    loop {
    let result = (|| -> std::io::Result<()> {
        engine.send("position startpos moves e2e4")?;
        engine.send("go movetime 1000")?;
        let reply = engine.receive("bestmove")?;
        let best_move = reply.split_whitespace().nth(1).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse best move")
        });
        println!("Black's reply: {}", best_move.unwrap_or("none"));
        engine.send("quit")?;
        Ok(())
    })();

    if result.is_err() {
        let _ = engine.stockfish_process.kill();
    }
    engine.stockfish_process.wait()?;
    result?;
    }
}
