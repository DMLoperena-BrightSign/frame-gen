use clap::{Parser};
use frames::FrameGen;

pub mod frames;

fn main() -> Result<(), String> {
    let framegen = FrameGen::parse();
    framegen.generate_frames()?;
    Ok(())
}
