extern crate stdshout;

use std::io;
use stdshout::Stdshout;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let mut stdout = Stdshout::new(stdout.lock());

    // Silently drop write errors
    let _ = io::copy(&mut stdin, &mut stdout);
}
