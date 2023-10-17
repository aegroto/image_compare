use image::{imageops, DynamicImage, RgbaImage};
use pixels::Pixels;
use winit::window;

pub fn draw(
    canvas: &mut Pixels,
    images: &Vec<DynamicImage>,
    (crop_width, crop_height): (u32, u32),
) {
    let mut result = RgbaImage::new(crop_width * images.len() as u32, crop_height);
    log::info!("Result: {} {}", result.width(), result.height());

    for (idx, image) in images.iter().enumerate() {
        let rgba_image = image.to_rgba8();
        imageops::replace(
            &mut result,
            &rgba_image,
            (idx as u32 * crop_width) as i64,
            0,
        );
    }

    let resized_result = imageops::resize(
        &result,
        canvas.texture().width(),
        canvas.texture().height(),
        imageops::FilterType::Nearest,
    );
    canvas
        .frame_mut()
        .copy_from_slice(&resized_result.as_flat_samples().samples);
}
