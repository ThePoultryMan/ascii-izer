#![cfg(feature = "crossterm")]

use std::{
    io::{stdout, Write},
    path::Path,
};

use crossterm::{
    cursor::MoveTo,
    style::{self, Print, SetForegroundColor},
    terminal::{size, Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use image::{imageops::FilterType, ImageReader};

use crate::{
    color::{Color, GrayscaleMode},
    image_into_lines, AsciiError,
};

pub fn put_in_console<P: AsRef<Path>>(
    image_path: P,
    #[cfg(feature = "color")] with_color: bool,
) -> Result<(), AsciiError> {
    let mut stdout = stdout();

    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.queue(MoveTo(0, 0));
    let _ = stdout.flush();
    let (width, height) = size().expect("Could not get size terminal.");
    let mut image = ImageReader::open(image_path)?.decode()?;
    image = image.resize_exact(width as u32, height as u32, FilterType::Lanczos3);

    let lines = image_into_lines(
        &image,
        GrayscaleMode::Luminosity,
        #[cfg(feature = "color")]
        false,
    )?;

    for line in lines {
        for i in 0..line.chars().len() {
            let _ = stdout.queue(Print(line.chars()[i]));
        }
        let _ = stdout.queue(Print("\n"));
    }

    let _ = stdout.queue(SetForegroundColor(style::Color::White));
    let _ = stdout.flush();

    Ok(())
}
