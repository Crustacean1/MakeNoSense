use image::RgbImage;

use super::triangle::Segmentation;

pub fn build_mask((width, height): (u32, u32), segments: &[Segmentation]) -> RgbImage {
    let mut image_buffer = RgbImage::new(width, height);

    segments.iter().for_each(|segment| {
        segment.triangles.iter().for_each(|triangle| {
            triangle.render(segment.color, image_buffer.rows_mut());
        });
    });

    image_buffer
}
