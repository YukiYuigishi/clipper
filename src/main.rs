use std::io::{self, Read};

use arboard::{Clipboard, SetExtWindows};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut buffer = Vec::new();

    handle.read_to_end(&mut buffer)?;
    let utf8_input = String::from_utf8_lossy(&buffer);

    let mut clipboard = Clipboard::new().unwrap();
    let set = clipboard.set();
    set.exclude_from_history().text(utf8_input).unwrap();
    Ok(())
}
