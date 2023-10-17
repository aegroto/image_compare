use image::{imageops, DynamicImage, RgbaImage};

use crate::ImageFocus;

pub fn process_images(images: &Vec<DynamicImage>, focus: &ImageFocus) -> Vec<DynamicImage> {
    log::debug!("Reprocessing images with focus: {:?}", focus);
    images
        .iter()
        .map(|image| process_image(&image, focus))
        .collect()
}

fn process_image(input: &DynamicImage, focus: &ImageFocus) -> DynamicImage {
    let mut output = DynamicImage::ImageRgba8(RgbaImage::new(input.width(), input.height()));

    let (x, replace_x, width) = apply_offset(input.width(), focus.x_offset);
    let (y, replace_y, height) = apply_offset(input.height(), focus.y_offset);
    imageops::replace(
        &mut output,
        &input.crop_imm(x, y, width, height),
        replace_x,
        replace_y,
    );

    output
}

fn apply_offset(measure: u32, offset: i32) -> (u32, i64, u32) {
    let crop_offset = offset.clamp(0, measure as i32) as u32;
    let replace_offset = if offset > 0 { 0 } else { offset.abs() } as i64;
    let crop_measure = (measure - offset.abs() as u32).clamp(0, measure);

    (crop_offset, replace_offset, crop_measure)
}
