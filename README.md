# fast dhash

A fast rust implementation of the perceptual hash ["dhash"](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html).

The main difference with other rust implementations, and the reason it is called "fast",
is that it doesn't use `grayscale` and `resize_exact` image methods, therefore running about ~50% faster

## basic usage

```rust
use fast_dhash::Dhash;

use image;
use std::path::Path;

fn main() {
    let path = Path::new("../image.jpg");
    let image = image::open(path);

    if let Ok(image) = image {
        let hash = Dhash::new(&image);
        println!("hash: {}", hash);
        // hash: d6a288ac6d5cce14
    }
}
```