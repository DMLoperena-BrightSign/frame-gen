use std::path::PathBuf;
use clap::{Parser};
use std::fs;
use image::{GenericImage, Rgb, ImageBuffer, RgbImage, imageops};
use num_cpus;
use rayon::ThreadPoolBuilder;

pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct FrameGen {
    #[arg(short = 't', long = "time", default_value_t = 10, required = false)]
    pub time: usize,
    #[arg(short = 'd', long = "dir", default_value = ".", required = false)]
    pub directory: PathBuf,
    #[arg(short = 'w', long = "width", default_value_t = 1920, required = false)]
    pub width: u32,
    #[arg(short = 'v', long = "height", default_value_t = 1080, required = false)]
    pub height: u32,
    #[arg(short = 'f', long = "fps", default_value_t = 60, required = false)]
    pub fps: u16,
    #[arg(short = 'b', long = "sweeper-height", default_value_t = 60, required = false)]
    pub sweeper_height: u32
}

impl FrameGen {
   
    // Create a base image which is just 8 colored rectangles split across the width
    // of the image. The rectangles will be the height of the image
    // Create a base sweeper which is a multi-color rectangle that stretches the width
    // of the image.

    fn create_base(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        // Create all 8 bars and color them. Once they are created we can copy them
        // to the base image at the give offset and then return the base image
        let width = self.width / 8;
        let white = ImageBuffer::from_pixel(width, self.height, Rgb([255, 255, 255]));
        let yellow = ImageBuffer::from_pixel(width, self.height, Rgb([255, 255, 0]));
        let lblue = ImageBuffer::from_pixel(width, self.height, Rgb([173, 216, 230]));
        let green = ImageBuffer::from_pixel(width, self.height, Rgb([0, 255, 0]));
        let purple = ImageBuffer::from_pixel(width, self.height, Rgb([128, 0, 128]));
        let red = ImageBuffer::from_pixel(width, self.height, Rgb([255, 0, 0]));
        let blue = ImageBuffer::from_pixel(width, self.height, Rgb([0, 0, 255]));
        let black = ImageBuffer::from_pixel(width, self.height, Rgb([0, 0, 0]));

        let mut full_image = RgbImage::new(self.width, self.height);
        full_image.copy_from(&white, 0, 0).unwrap();
        full_image.copy_from(&yellow, width, 0).unwrap();
        full_image.copy_from(&lblue, width * 2, 0).unwrap();
        full_image.copy_from(&green, width * 3, 0).unwrap();
        full_image.copy_from(&purple, width * 4, 0).unwrap();
        full_image.copy_from(&red, width * 5, 0).unwrap();
        full_image.copy_from(&blue, width * 6, 0).unwrap();
        full_image.copy_from(&black, width * 7, 0).unwrap();
        return full_image;
    }

    fn create_sweeper(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let width = self.width / 8;
        let white = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([255, 255, 255]));
        let yellow = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([255, 255, 0]));
        let lblue = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([173, 216, 230]));
        let green = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([0, 255, 0]));
        let purple = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([128, 0, 128]));
        let red = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([255, 0, 0]));
        let blue = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([0, 0, 255]));
        let black = ImageBuffer::from_pixel(width, self.sweeper_height, Rgb([0, 0, 0]));

        let mut full_sweeper = RgbImage::new(self.width, self.sweeper_height);
        full_sweeper.copy_from(&black, 0, 0).unwrap();
        full_sweeper.copy_from(&blue, width, 0).unwrap();
        full_sweeper.copy_from(&red, width * 2, 0).unwrap();
        full_sweeper.copy_from(&purple, width * 3, 0).unwrap();
        full_sweeper.copy_from(&green, width * 4, 0).unwrap();
        full_sweeper.copy_from(&lblue, width * 5, 0).unwrap();
        full_sweeper.copy_from(&yellow, width * 6, 0).unwrap();
        full_sweeper.copy_from(&white, width * 7, 0).unwrap();
        return full_sweeper;
    }

    fn draw_frame(&self, base: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, sweeper: ImageBuffer<Rgb<u8>, Vec<u8>>, frame_num: usize, position: u32) {
        imageops::overlay(base, &sweeper, 0 as i64, position as i64);
        let file = self.directory.join(format!("frame{}.png", frame_num));
        base.save(file).unwrap();
    } 

    pub fn generate_frames(&self) -> Result<(), String> {
        // Try to create the specified directory and bail early if we can't
        self.create_directory()?;
        let cores = num_cpus::get();
        let pool = ThreadPoolBuilder::new().num_threads(cores).build().unwrap();
        pool.in_place_scope(|s| {
            let delta = (self.height - self.sweeper_height)/self.fps as u32;
            let mut position: u32 = 0;
            let iters: usize = self.time * self.fps as usize;
            let mut direction = Direction::Down;
            let base = self.create_base();
            let sweeper = self.create_sweeper();
            for x in 0..iters {
                let mut b_clone = base.clone();
                let s_clone = sweeper.clone();
                s.spawn(move |_| self.draw_frame(&mut b_clone, s_clone, x, position));
                match direction {
                    Direction::Up => {
                        position = position - delta;
                        if position <= 0 {
                            position = 0;
                            direction = Direction::Down;
                        }
                    }
                    Direction::Down => {
                        position = position + delta;
                        if position >= self.height - self.sweeper_height {
                            position = self.height - self.sweeper_height;
                            direction = Direction::Up;
                        }
                    }
                }
            }
            
        });
        Ok(())
    }

    // Helper method. Might move it elsewhere
    fn create_directory(&self) -> Result<(), String> {
        match fs::create_dir_all(&self.directory) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(format!("Failed to create directory: {:?}", e));
            }
        }
    }
}
