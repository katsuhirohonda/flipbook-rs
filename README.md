# flipbook-rs

A Rust command-line tool to easily create GIF animations from image files.

## Overview

flipbook-rs is a simple tool that creates animated GIFs from JPEG/PNG files in a specified directory. Files are automatically sorted by modification time to generate smooth animations.

## Features

- Create GIF animations from JPG/JPEG/PNG images
- Automatic sorting by file modification time
- Customizable delay between frames
- Custom output filename
- Automatic resizing of all images to the same dimensions
- Infinite loop setting

## Prerequisites

- Rust and Cargo installed

## Installation

### From crates.io

Install directly from crates.io:

```
cargo install flipbook-rs
```

This will add the `flipbook` command to your PATH.

### From Source

Clone the repository and build the project:

```
git clone https://github.com/katsuhirohonda/flipbook-rs.git
cd flipbook-rs
cargo build --release
```

The compiled binary will be available at `target/release/flipbook`.

## Usage

Basic usage:

```
flipbook [DIRECTORY] -o [OUTPUT_FILE] -d [DELAY]
```

### Examples

Create a GIF from images in the current directory:
```
flipbook
```

Create a GIF from images in a specific directory:
```
flipbook path/to/images
```

Specify a custom output filename:
```
flipbook -o animation.gif
```

Customize the delay between frames (in 1/100 seconds):
```
flipbook -d 20
```

Combine all options:
```
flipbook path/to/images -o my_animation.gif -d 5
```

## Parameters

| Parameter | Description | Default Value |
|-----------|-------------|---------------|
| dir | Directory containing image files | Current directory (`.`) |
| -o, --output | Path to the output GIF file | `output.gif` |
| -d, --delay | Delay between frames (in 1/100 seconds) | 10 |

## Supported Formats

- JPG/JPEG
- PNG

## How It Works

1. Searches for JPG/JPEG/PNG images in the specified directory
2. Sorts files by modification time
3. Determines GIF dimensions based on the first image
4. Resizes each image to the same dimensions
5. Generates a GIF animation with the specified delay
6. Outputs an infinitely looping GIF file

## Dependencies

- clap: Command line argument parsing
- gif: GIF file encoding
- image: Image processing
- anyhow: Error handling

## License

[MIT LICENSE](LICENSE) 

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
