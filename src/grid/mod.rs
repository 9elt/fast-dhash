mod luma;

use image::DynamicImage;

/// ### grid from image
/// sums the grayscaled pixel values into a 2D array
///
/// #### performance
/// *(in release mode)* ~50% faster than using `grayscale`  
/// and `resize_exact` *(with linear filter)* image methods
///
/// #### paincs
/// panics with images bigger than 34,952 x 34,952 (~1.2 billion pixels)
///
pub fn from_image(image: &DynamicImage) -> [[u32; 9]; 8] {
    let width = image.width() as usize;
    let height = image.height() as usize;

    let color = image.color();

    let n_channels = color.channel_count() as usize;

    let bytes = image.as_bytes();

    let cell_width = (width / 9) as usize;
    let cell_height = (height / 8) as usize;

    let props: [usize; 4] = [width, cell_width, cell_height, n_channels];

    match n_channels > 2 {
        true => from_rgb_bytes(bytes, props),
        false => from_luma_bytes(bytes, props),
    }
}

fn from_rgb_bytes(
    bytes: &[u8],
    [width, cell_width, cell_height, n_channels]: [usize; 4],
) -> [[u32; 9]; 8] {
    let mut grid: [[u32; 9]; 8] = [[0u32; 9]; 8];

    // loop through every pixel avoiding the
    // divisions to calculate grid indexes
    for grid_x in 0..9 {
        let offset = grid_x * cell_width;
        for img_x in offset..offset + cell_width {
            for grid_y in 0..8 {
                let offset = grid_y * cell_height;
                for img_y in offset..offset + cell_height {
                    // calculate the current index
                    let i = (img_y * width + img_x) * n_channels;

                    grid[grid_y][grid_x] +=
                        // rgb to luma conversion
                        luma::from_rgb(bytes[i], bytes[i + 1], bytes[i + 2]) as u32
                }
            }
        }
    }

    grid
}

fn from_luma_bytes(
    bytes: &[u8],
    [width, cell_width, cell_height, n_channels]: [usize; 4],
) -> [[u32; 9]; 8] {
    let mut grid: [[u32; 9]; 8] = [[0u32; 9]; 8];

    // loop through every pixel avoiding the
    // divisions to calculate grid indexes
    for grid_x in 0..9 {
        let offset = grid_x * cell_width;
        for img_x in offset..offset + cell_width {
            for grid_y in 0..8 {
                let offset = grid_y * cell_height;
                for img_y in offset..offset + cell_height {
                    // calculate the current index
                    let i = (img_y * width + img_x) * n_channels;

                    grid[grid_y][grid_x] += bytes[i] as u32
                }
            }
        }
    }

    grid
}
