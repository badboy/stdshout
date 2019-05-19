//! Shout it out!
//!
//! ## Example
//!
//! ```
//! assert_eq!("I AM NOT SHOUTING!!!1!", stdshout::shout("i am not shouting"));
//! ```

use std::io;
use std::io::Write;

const APPEND: &str = "!!!1!";

pub struct Stdshout<W: Write>(W);

impl<W: Write> Stdshout<W> {
    pub fn new(w: W) -> Stdshout<W> {
        Stdshout(w)
    }
}

impl<W: Write> Write for Stdshout<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let lossy = String::from_utf8_lossy(buf).to_uppercase();
        for line in lossy.split('\n') {
            if line.len() > 0 {
                self.0.write_all(line.as_bytes())?;
                self.0.write_all(APPEND.as_bytes())?;
            }
            self.0.write_all(b"\n")?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

/// Interpret data as UTF-8 and shout it out.
pub fn shout<D: AsRef<[u8]>>(data: D) -> String {
    let mut lossy = String::from_utf8_lossy(data.as_ref()).to_uppercase();
    lossy.push_str(APPEND);
    lossy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("HELLO!!!1!", shout("hello"));
    }

    #[test]
    fn binary_data() {
        assert_eq!("�!!!1!", shout(b"\xF0\x90\x80"));
    }
}
