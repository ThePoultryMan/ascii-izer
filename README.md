# ascii-izer

An image to ASCII art library for rust

## Features

- Color \[default\]: Processes the image color alongside the pixels during ASCII generation.
- Crossterm: Provides a simple method to put the ASCII into the console via crossterm. This does not output color

## Usage

### Simple

The functions [to_ascii_lines](https://docs.rs/ascii-izer/latest/ascii_izer/fn.to_ascii_lines.html) and [image_into_lines](https://docs.rs/ascii-izer/latest/ascii_izer/fn.image_into_lines.html) are the simplest way to convert an image into ASCII. Keep in mind, however, the dimensions taken by these functions are the exact level that the input image will be resized to, aspect ration is not respected.

### Advanced

The [ASCIIGenerator](https://docs.rs/ascii-izer/latest/ascii_izer/struct.ASCIIGenerator.html) struct can be used to have more control over the exact parameters used.
