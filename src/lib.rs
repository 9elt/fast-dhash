//! # Fast Dhash
//!
//! A fast rust implementation of the perceptual hash [*dhash*](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html).
//!
//! The main difference with other rust implementations, and the reason it is called *fast*, is that it uses multithreading and does not resize nor converts the image, effectively cycling through its bytes only once.
//!
//! ## Usage
//!
//! For forward and backward compatibility, the API does NOT directly rely on the [*image*](https://docs.rs/image/latest/image/index.html) crate.
//!
//! ```
//! use fast_dhash::Dhash;
//! use image::ImageReader;
//!
//! let image = ImageReader::open("./image.jpg")
//!     .expect("cannot read image")
//!     .decode()
//!     .expect("cannot decode image");
//!
//! let hash = Dhash::new(
//!     image.as_bytes(),
//!     image.width(),
//!     image.height(),
//!     image.color().channel_count(),
//! );
//!
//! println!("hash: {}", hash);
//! // hash: d6a288ac6d5cce14
//! ```
use serde::{Deserialize, Serialize};
use std::{fmt, num, str, thread};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Dhash {
    hash: u64,
}

impl Dhash {
    pub fn new(bytes: &[u8], width: u32, heigth: u32, channel_count: u8) -> Self {
        let width = width as usize;
        let heigth = heigth as usize;
        let channel_count = channel_count as usize;

        // NOTE: Very important, prevents possible segfault
        if width * heigth * channel_count != bytes.len() {
            panic!(
                "Invalid image dimensions, expected {} got {}",
                width * heigth * channel_count,
                bytes.len()
            );
        }

        let cell_width = width / 9;
        let cell_height = heigth / 8;

        let grid = if channel_count >= 3 {
            grid_from_rgb(bytes, width, cell_width, cell_height, channel_count)
        } else {
            grid_from_grayscale(bytes, width, cell_width, cell_height, channel_count)
        };

        let mut bits = [false; 64];

        for y in 0..8 {
            for x in 0..8 {
                bits[y * 8 + x] = grid[y][x] > grid[y][x + 1];
            }
        }

        let mut hash: u64 = 0;

        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                hash += 1 << i;
            }
        }

        Self { hash }
    }

    pub fn from_u64(hash: u64) -> Self {
        Self { hash }
    }

    pub fn hamming_distance(&self, other: &Self) -> u32 {
        (self.hash ^ other.hash).count_ones()
    }

    pub fn to_u64(&self) -> u64 {
        self.hash
    }
}

impl PartialEq for Dhash {
    fn eq(&self, other: &Self) -> bool {
        self.hamming_distance(other) < 11
    }
}

impl fmt::Display for Dhash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", &self.hash)
    }
}

impl str::FromStr for Dhash {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match u64::from_str_radix(s, 16) {
            Ok(hash) => Ok(Self { hash }),
            Err(error) => Err(error),
        }
    }
}

fn grid_from_rgb(
    bytes: &[u8],
    width: usize,
    cell_width: usize,
    cell_height: usize,
    channel_count: usize,
) -> [[f64; 9]; 8] {
    let mut grid = [[0f64; 9]; 8];

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(8);

        for y in 0..8 {
            handles.push(s.spawn(move || {
                let mut row = [0f64; 9];

                for (x, cell) in row.iter_mut().enumerate() {
                    let from = x * cell_width;
                    let to = from + cell_width;

                    let mut rs = 0f64;
                    let mut gs = 0f64;
                    let mut bs = 0f64;

                    for image_x in from..to {
                        let from = y * cell_height;
                        let to = from + cell_height;

                        for image_y in from..to {
                            let i = (image_y * width + image_x) * channel_count;

                            unsafe {
                                rs += *bytes.get_unchecked(i) as f64;
                                gs += *bytes.get_unchecked(i + 1) as f64;
                                bs += *bytes.get_unchecked(i + 2) as f64;
                            }
                        }
                    }

                    *cell += rs * 0.299 + gs * 0.587 + bs * 0.114;
                }

                (y, row)
            }));
        }

        for handle in handles {
            let (y, row) = handle.join().unwrap();
            grid[y] = row;
        }
    });

    grid
}

fn grid_from_grayscale(
    bytes: &[u8],
    width: usize,
    cell_width: usize,
    cell_height: usize,
    channel_count: usize,
) -> [[f64; 9]; 8] {
    let mut grid = [[0f64; 9]; 8];

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(8);

        for y in 0..8 {
            handles.push(s.spawn(move || {
                let mut row = [0f64; 9];

                for (x, cell) in row.iter_mut().enumerate() {
                    let from = x * cell_width;
                    let to = from + cell_width;

                    let mut luma = 0f64;

                    for image_x in from..to {
                        let from = y * cell_height;
                        let to = from + cell_height;

                        for image_y in from..to {
                            let i = (image_y * width + image_x) * channel_count;

                            unsafe {
                                luma += *bytes.get_unchecked(i) as f64;
                            }
                        }
                    }

                    *cell += luma;
                }

                (y, row)
            }));
        }

        for handle in handles {
            let (y, row) = handle.join().unwrap();
            grid[y] = row;
        }
    });

    grid
}
