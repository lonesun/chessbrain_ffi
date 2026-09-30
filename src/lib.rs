use std::{io, process};
use std::io::{BufRead, Write};

pub struct Engine {
	stockfish_process: std::process::Child,
	output: io::BufReader<std::process::ChildStdout>,
}

impl Engine {
	pub fn start(path: &str) -> io::Result<Self> {
		let mut stockfish_process = std::process::Command::new("Stockfish/stockfish-windows-x86-64-universal.exe")
			.stdin(std::process::Stdio::piped())
			.stdout(std::process::Stdio::piped())
			.stderr(std::process::Stdio::piped())
			.spawn()?;

		let output = io::BufReader::new(stockfish_process.stdout.take().expect("Failed to take stdout"));

		let mut engine = Self { stockfish_process, output };

        /* Initialise communication with Stockfish */
        engine.send("uci")?; // UCI = Universal Chess Interface
        engine.receive("uciok")?; // Acknowledgement from Stockfish
        engine.send("isready")?; // Check if Stockfish is ready
        engine.receive("readyok")?; // Readiness confirmation


		Ok(engine)
	}

    pub fn send(&mut self, command: &str) -> io::Result<()> {
        let input = self.stockfish_process.stdin.as_mut().expect("Failed to take stdin");
	    writeln!(input, "{}", command)?;
        input.flush() // Produces its own io::Result<()>
    }

    pub fn receive(&mut self, token: &str) -> io::Result<String> {
        loop {
            let mut line = String::new();
            if self.output.read_line(&mut line)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "End of input"));
            }
            if line.split_whitespace().next() == Some(token) {
                return Ok(line);
            }
        }
    }
}
