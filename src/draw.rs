use image::DynamicImage;
use pixels::Pixels;
use winit::window;

pub fn draw(canvas: &mut Pixels, images: &Vec<DynamicImage>, (crop_width, crop_height): (u32, u32)) {
    let crop_width = crop_width as usize;
    let crop_height = crop_height as usize;

    let window_pixels = canvas.frame_mut();
    window_pixels.fill(255u8);
    for (idx, image) in images.iter().enumerate() {
        let image_pixels = image.as_flat_samples_u8().unwrap().samples;
        for ix in 0..image.width() as usize {
            for iy in 0..image.height() as usize {
                let wx = idx * crop_width + ix;
                let wy = idx * crop_height + iy;

                for channel in 0..4 {
                    window_pixels[wx * crop_width + wy + channel] =
                        image_pixels[ix * crop_width + iy + channel];
                }
            }
        }
    }
}
