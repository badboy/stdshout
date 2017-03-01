//! Shout it out!
//!
//! ## Example
//!
//! ```
//! assert_eq!("I AM NOT SHOUTING!!!1!", stdshout::shout("i am not shouting"));
//! ```

const APPEND : &'static str = "!!!1!";

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
