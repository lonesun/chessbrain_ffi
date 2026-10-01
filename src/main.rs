use chessbrain_ffi::{Engine};

fn main() -> std::io::Result<()> {
    let mut engine = Engine::start("Stockfish/stockfish-windows-x86-64-universal.exe")?;
    let mut history = vec!["e2e4"];

    loop {
		let result = (|| -> std::io::Result<()> {
			engine.send(&format!("position startpos moves {}", history.join(" ")))?;
			engine.send("go movetime 1000")?;
			let reply = engine.receive("bestmove")?;
			let best_move = reply.split_whitespace().nth(1).ok_or_else(|| {
				std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse best move")
			});
			if let Ok(best_move) = best_move {
				history.push(best_move);
				println!("Black's reply: {}", best_move);
			} else {
				println!("Black's reply: none");
			}
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
