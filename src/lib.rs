use cxx::bridge;
use cxx::{CxxString, CxxVector};

/**
 * Q: How is a normal Rust `String`` similar to a `CxxString`?
 * A: Both represent sequences of UTF-8 encoded bytes, but `CxxString` is used for interoperability with C++ code.
 * Q: Why can't a normal Rust `String` be directly converted to a `CxxVector<u8>`?
 * A: Because `CxxVector<u8>` is designed for interoperability with C++ code, and a normal Rust `String` does not have the same memory layout as a `CxxString`.
 */

fn pipe(input: &CxxString) -> CxxVector<u8> {
    let mut output = CxxVector::new();
    for &byte in input.as_bytes() {
        output.push(byte);
    }
    output
}
