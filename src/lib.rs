//! ## fast dhash
//!
//! A fast rust implementation of the perceptual hash ["dhash"](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html).
//! 
//! The main difference with other rust implementations, and the reason it is called "fast",
//! is that it doesn't use `grayscale` and `resize_exact` image methods, therefore running about ~50% faster
//! 
//! ### basic usage
//!
//! ```
//! use fast_dhash::Dhash;
//!
//! use image;
//! use std::path::Path;
//!
//! fn main() {
//!     let path = Path::new("../image.jpg");
//!     let image = image::open(path);
//! 
//!     if let Ok(image) = image {
//!         let hash = Dhash::new(&image);
//!         println!("hash: {}", hash);
//!         // hash: d6a288ac6d5cce14
//!     }
//! }
//! ```

mod grid;

use image::DynamicImage;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;

/// ## Dhash
/// 
/// ### basic usage
///
/// ```
/// use fast_dhash::Dhash;
///
/// use image;
/// use std::path::Path;
///
/// fn main() {
///     let path = Path::new("../image.jpg");
///     let image = image::open(path);
/// 
///     if let Ok(image) = image {
///         let hash = Dhash::new(&image);
///         println!("hash: {}", hash);
///         // hash: d6a288ac6d5cce14
///     }
/// }
/// ```
/// 
/// ### comparaison
/// 
/// Dhash implements `PartialEq` instead of `Eq` to match
/// cases where the compared hashes represent a variation
/// of the same image.
///
/// to check the equality use
/// `hash.hamming_distance(&other_hash) == 0`
/// 
/// #### example
/// ```
/// use fast_dhash::Dhash;
/// 
/// fn main() {
///     let hash = Dhash::from_str("d6a288ac6d5cce14").unwrap();
///     let similar = Dhash::from_str("d6a088ac6d5cce14").unwrap();
///     let different = Dhash::from_str("a63ebccdfd5dbfff").unwrap();
/// 
///     assert!(hash == similar);
///     assert!(hash.hamming_distance(&similar) == 1);
/// 
///     assert!(hash != different);
///     assert!(hash.hamming_distance(&different) == 26);
/// }
/// 
/// ```
///
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Dhash {
    hash: u64,
}

impl Dhash {
    /// ### dhash algorithm
    ///
    /// generates a new dhash from a `DynamicImage`
    ///
    /// #### paincs
    /// panics with images bigger than 34,952 x 34,952 (~1.2 billion pixels)
    ///
    pub fn new(image: &DynamicImage) -> Self {
        let grid = grid::from_image(image);

        let mut hash_bits: Vec<bool> = Vec::with_capacity(64);

        for y in 0..8 {
            for x in 0..8 {
                hash_bits.push(grid[y][x] > grid[y][x + 1])
            }
        }

        let mut hash: u64 = 0;

        for i in 0..64 {
            if hash_bits[i] {
                hash += 1 << i
            }
        }

        Self { hash }
    }

    /// ### Dhash from `String`
    ///
    /// converts an hex `String` to `Dhash`
    ///
    /// #### example
    /// ```
    /// use fast_dhash::Dhash;
    ///
    /// fn main() {
    ///     let string = "d6a288ac6d5cce14".to_string();
    ///
    ///     let hash = Dhash::from_string(&string).unwrap();
    ///     let hash_u64 = hash.to_u64();
    ///
    ///     assert_eq!(hash_u64, 15466074344494255636u64);
    /// }
    /// ```
    pub fn from_string(hash_hex: &String) -> Result<Self, ParseIntError> {
        match u64::from_str_radix(&hash_hex, 16) {
            Ok(hash) => Ok(Self { hash }),
            Err(error) => Err(error),
        }
    }

    /// ### Dhash from `str`
    ///
    /// converts an hex `str` to `Dhash`
    ///
    /// #### example
    /// ```
    /// use fast_dhash::Dhash;
    ///
    /// fn main() {
    ///     let hash = Dhash::from_str("d6a288ac6d5cce14").unwrap();
    ///     let hash_u64 = hash.to_u64();
    ///
    ///     assert_eq!(hash_u64, 15466074344494255636u64);
    /// }
    /// ```
    pub fn from_str(hash_hex: &str) -> Result<Self, ParseIntError> {
        match u64::from_str_radix(hash_hex, 16) {
            Ok(hash) => Ok(Self { hash }),
            Err(error) => Err(error),
        }
    }

    /// ### Dhash from u64 hash
    ///
    /// parse a `u64`
    /// 
    pub fn from_u64(hash: u64) -> Self {
        Self { hash }
    }

    /// ### dhash hamming distance
    ///
    /// compares two hashes and returns a value between `0` and `64`
    ///
    /// `0` identical
    ///
    /// `1~10` variation
    ///
    /// `>10` different
    ///
    pub fn hamming_distance(&self, other: &Self) -> u32 {
        (&self.hash ^ other.hash).count_ones()
    }

    /// ### u64 hash
    ///
    /// returns the `u64` hash
    ///
    pub fn to_u64(&self) -> u64 {
        self.hash
    }
}

// PartialEq
//
// Dhash implements `PartialEq` instead of `Eq` to match
// cases where the compared hashes represent a variation
// of the same image.
//
// to check the equality use
// `hash.hamming_distance(&other_hash) == 0`
//
impl PartialEq for Dhash {
    fn eq(&self, other: &Self) -> bool {
        self.hamming_distance(other) < 11
    }
}

// Display
//
// Dhash is displayed as an hex string
//
impl fmt::Display for Dhash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", &self.hash)
    }
}
