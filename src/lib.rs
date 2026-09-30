use std::io;

pub struct Engine;

impl Engine {
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
