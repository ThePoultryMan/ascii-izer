#![cfg(feature = "crossterm")]

use std::{
    io::{stdout, Write},
    path::Path,
};

use crossterm::{
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};

use crate::{to_ascii_string, AsciiError};

pub fn put_in_console<P: AsRef<Path>>(image: P) -> Result<(), AsciiError> {
    let mut stdout = stdout();

    let _ = stdout.execute(Clear(ClearType::All));
    let (width, height) = size().expect("Could not get size terminal.");

    let ascii = to_ascii_string(image, (width as u32, height as u32))?;

    for (index, chars) in ascii.iter().enumerate() {
        let _ = stdout.write(chars.as_bytes());
        if index != ascii.len() - 1 {
            let _ = stdout.write(b"\n");
        }
    }

    Ok(())
}
