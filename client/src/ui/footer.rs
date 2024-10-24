use std::io::{self, Write};
use crossterm::terminal::size;

pub fn draw() {
    let (_width, height) = size().unwrap();

    print!("\x1B[{};1H$", height);
    io::stdout().flush().unwrap();
}