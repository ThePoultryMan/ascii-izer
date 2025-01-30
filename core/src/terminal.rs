#![cfg(feature = "crossterm")]

use std::{
    io::{stdout, Write},
    path::Path,
};

use crossterm::{
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};

use crate::{to_ascii, AsciiError};

pub fn put_in_console<P: AsRef<Path>>(image: P) -> Result<(), AsciiError> {
    let mut stdout = stdout();

    let _ = stdout.execute(Clear(ClearType::All));
    let (width, height) = size().expect("Could not get size terminal.");

    let ascii = to_ascii(image, (width as u32, height as u32))?;

    let chunks = ascii.chunks(width.into()).enumerate();
    let chunks_max = chunks.len() - 1;
    for (index, chars) in chunks {
        let _ = stdout.write(chars.iter().collect::<String>().as_bytes());
        if index != chunks_max {
            let _ = stdout.write(b"\n");
        }
    }

    Ok(())
}
