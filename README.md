# pepeline-rs

Fork of [scanlate-wiki/pepeline-rs](https://github.com/scanlate-wiki/pipeline-rs) for pure Rust usage

## Add to your project

```
cargo add --git https://github.com/ivabus/pepeline-rs
```

## Examples

Apply screentone

```rust
extern crate image;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};

let img = image::open("page.png").unwrap();
let dot_size = 8;
let angle = 0.0;
let dot_type = pepeline::core::enums::TypeDot::CIRCLE;
let (w, h) = img.dimensions();
let mut ndar = pepeline::core::convert::luma2arrayf32(img.to_luma8());
if angle == 0.0 {
    pepeline::halftone::screentone_add::screentone_add(&mut ndar, dot_size, dot_type);
} else {
    pepeline::halftone::screentone_add::screentone_rotate_add(
        &mut ndar, dot_size, angle, dot_type,
    );
}
let image = ImageBuffer::<Luma<f32>, Vec<f32>>::from_raw(w, h, ndar.into_raw_vec()).unwrap();
DynamicImage::from(image)
    .to_luma8()
    .save("toned.png")
    .unwrap();
```