use std::io;

pub fn send(writer: &mut dyn io::Write, command: &str) -> io::Result<()> {
    writeln!(writer, "{}", command)?;
    // writer.write_all(command.as_bytes())?;
    writer.flush()
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

#[allow(unused_imports)]
use cxx::bridge;
#[allow(unused_imports)]
use cxx::{CxxString, CxxVector};

/*
trait CxxConvertible {
    fn to_cxx_string(&self) -> CxxString;
}

impl CxxConvertible for String {
    fn to_cxx_string(&self) -> CxxString {
        CxxString::from(self.as_str())
    }
}

fn pipe(input: &CxxString) -> CxxVector<u8> {
    let mut output = CxxVector::new();
    for &byte in input.as_bytes() {
        output.pin_mut().push(byte);
    }
    output
}
*/
