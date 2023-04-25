use image::RgbImage;

use crate::editor::ui_layer::UiLayer;

pub fn build_mask(
    (width, height): (u32, u32),
    vertices: &Vec<(f32, f32)>,
    layers: &[UiLayer],
) -> RgbImage {
    let mut image_buffer = RgbImage::new(width, height);
    let (mut x, mut y) = (-(width as i32 / 2), (height as i32 / 2));
    image_buffer.rows_mut().for_each(|row| {
        x = -(width as i32 / 2);
        row.for_each(|pixel| {
            pixel.0 = sample_pixel((x as f32, y as f32), vertices, layers);
            x += 1;
        });
        y -= 1;
    });
    image_buffer
}

fn sample_pixel((x, y): (f32, f32), vertices: &Vec<(f32, f32)>, layers: &[UiLayer]) -> [u8; 3] {
    let layer_types = [
        (String::from("Masking"), [0, 255, 0]),
        (String::from("Non masking"), [0, 0, 255]),
        (String::from("Foreground"), [0, 0, 0]),
        (String::from("Animal"), [255, 0, 0]),
    ];

    let pixel_value = layer_types.iter().find(|layer_type| {
        layers
            .iter()
            .filter(|layer| layer.layer_info().layer_type.eq(&layer_type.0))
            .any(|layer| layer.contains(vertices, (x, y)))
    });

    match pixel_value {
        Some(pixel) => pixel.1,
        None => [0, 0, 0],
    }
}
