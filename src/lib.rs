use std::io;

pub struct Engine;

impl Engine {
    pub fn send(writer: &mut dyn io::Write, command: &str) -> io::Result<()> {
        writeln!(writer, "{}", command)?;
        writer.flush() // Produces its own io::Result<()>
    }

    pub fn receive(reader: &mut dyn io::BufRead, token: &str) -> io::Result<String> {
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "End of input"));
            }
            if line.split_whitespace().next() == Some(token) {
                return Ok(line);
            }
        }
    }
}
