# Fast Dhash

A fast rust implementation of the perceptual hash [*dhash*](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html).

The main difference with other rust implementations, and the reason it is called *fast*, is that it uses multithreading and does not resize nor converts the image, effectively cycling through its bytes only once.

## Usage

For forward and backward compatibility, the API does NOT directly rely on the [*image*](https://docs.rs/image/latest/image/index.html) crate.

```rust
use fast_dhash::Dhash;
use image::ImageReader;

let image = ImageReader::open("./image.jpg")
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
// hash: d6a288ac6d5cce14
```
