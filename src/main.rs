use anyhow::Result;
use clap::Parser;
use gif::{Encoder, Frame, Repeat};
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    dir: PathBuf,
    #[arg(short, long, default_value = "output.gif")]
    output: PathBuf,
    #[arg(short, long, default_value_t = 10)]
    delay: u16,
}

/// Main function
///
/// # Arguments
///
/// * `args` - The arguments passed to the program
///
/// # Returns
///
/// * `Result<()>` - The result of the program
///
/// # Errors
///
/// * `anyhow::Error` - If the program cannot be run
fn main() -> Result<()> {
    let args = Args::parse();
    let mut files: Vec<_> = fs::read_dir(&args.dir)?
        .filter_map(Result::ok)
        .filter(|e| {
            let ext = e
                .path()
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            matches!(ext.as_str(), "jpg" | "jpeg" | "png")
        })
        .collect();

    files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());

    let first = &files[0].path();
    let (width, height) = get_dimensions(first)?;
    let mut encoder = Encoder::new(fs::File::create(&args.output)?, width, height, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    for entry in files {
        let path = entry.path();
        let frames = match path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap()
            .to_lowercase()
            .as_str()
        {
            _ => vec![load_image(&path, width, height)?],
        };
        for mut frame in frames {
            frame.delay = args.delay;
            encoder.write_frame(&frame)?;
        }
    }

    println!("✅ Created {}", args.output.display());
    Ok(())
}

/// Get the dimensions of the image
///
/// # Arguments
///
/// * `path` - The path to the image
///
/// # Returns
///
/// * `(width, height)` - The dimensions of the image
///
/// # Errors    
///
/// * `ImageError` - If the image cannot be opened or resized
fn get_dimensions(path: &PathBuf) -> Result<(u16, u16)> {
    let img = image::open(path)?;
    Ok((img.width() as u16, img.height() as u16))
}

/// Load the image and resize it to the given dimensions
///
/// # Arguments
///
/// * `path` - The path to the image
/// * `w` - The width of the image
/// * `h` - The height of the image
///
/// # Returns
///
/// * `Frame` - The frame of the image
///
/// # Errors
///
/// * `ImageError` - If the image cannot be opened or resized
fn load_image(path: &PathBuf, w: u16, h: u16) -> Result<Frame> {
    let img = image::open(path)?
        .resize_exact(w.into(), h.into(), image::imageops::FilterType::Nearest)
        .to_rgb8();
    Ok(Frame::from_rgb(w, h, &img))
}
