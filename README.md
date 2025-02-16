# Fast Dhash

A fast rust implementation of the perceptual hash [*dhash*](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html).

The main difference with other rust implementations, and the reason it is called *fast*, is that it uses multithreading and does not resize nor converts the image, effectively cycling through its bytes only once.

## Usage

For forward and backward compatibility, *fast dhash* does NOT directly rely on the [*image*](https://docs.rs/image/latest/image/index.html) crate, it is up to the user to provide the image bytes and dimensions.

```rust
use fast_dhash::Dhash;
use image::ImageReader;

let image = ImageReader::open(".test/radial.jpg")
    .expect("cannot read image")
    .decode()
    .expect("cannot decode image");

let hash = Dhash::new(
    image.as_bytes(),
    image.width(),
    image.height(),
    image.color().channel_count(),
);

println!("hash: {}", hash);
// hash: f0f0e8cccce8f0f0
```
