use std::io;

fn send(writer: &mut dyn io::Write, command: &str) -> io::Result<()> {
    writeln!(writer, "{}", command)?;
    // writer.write_all(command.as_bytes())?;
    writer.flush()
}

fn receive(reader: &mut dyn io::BufRead) -> io::Result<String> {
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

use cxx::bridge;
use cxx::{CxxString, CxxVector};

#[allow(unused_doc_comments)]
/**
 * Q: How is a normal Rust `String`` similar to a `CxxString`?
 * A: Both represent sequences of UTF-8 encoded bytes, but `CxxString` is used for interoperability with C++ code.
 * Q: Why can't a normal Rust `String` be directly converted to a `CxxVector<u8>`?
 * A: Because `CxxVector<u8>` is designed for interoperability with C++ code, and a normal Rust `String` does not have the same memory layout as a `CxxString`.
 * Q: Presumably, in order to have Rust be involved, then `Strings` need to be converted to `CxxString` first.
 * A: Yes, because `CxxString` is the type that can be safely passed to C++ code, and only then can it be converted to a `CxxVector<u8>`.
 */

trait CxxConvertible {
    fn to_cxx_string(&self) -> CxxString;
}

impl CxxConvertible for String {
    fn to_cxx_string(&self) -> CxxString {
        CxxString::from(self)
    }
}

fn pipe(input: &CxxString) -> CxxVector<u8> {
    let mut output = CxxVector::new();
    for &byte in input.as_bytes() {
        output.push(byte);
    }
    output
}
