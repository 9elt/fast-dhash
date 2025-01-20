# Fast DHash

A fast rust implementation of the perceptual hash [*dhash*](https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html).

The main difference with other rust implementations, and the reason it is called *fast*, is that it uses multi-threading and doesn't rely internally on the [*image*](https://docs.rs/image/latest/image/index.html) crate methods.

## Usage

```rust
use fast_dhash::Dhash;
use image::open;
use std::path::Path;

let path = Path::new("../image.jpg");
let image = open(path);

if let Ok(image) = image {
    let hash = Dhash::new(
        image.as_bytes(),
        image.width(),
        image.height(),
        image.color().channel_count(),
    );
    println!("hash: {}", hash);
    // hash: d6a288ac6d5cce14
}
```
