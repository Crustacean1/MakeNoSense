use image::{buffer::RowsMut, Rgb};

use crate::{
    editor::{bounded_rect::BoundingBox, ui_layer::UiLayer},
    triangulator::{contains_triangle, Vec2},
};

pub struct Segmentation {
    pub vertices: Vec<f32>,
    pub triangles: Vec<Triangle>,
    pub id: usize,
    pub type_id: usize,
    pub color: [u8; 3],
}

#[derive(Debug)]
pub struct Triangle {
    vertices: [Vec2; 3],
    bounding_box: BoundingBox<f32>,
}

impl Triangle {
    pub fn new(indices: &[u32; 3], vertices: &[(f32, f32)]) -> Self {
        let vertices = [
            Vec2::from(vertices[indices[0] as usize]),
            Vec2::from(vertices[indices[1] as usize]),
            Vec2::from(vertices[indices[2] as usize]),
        ];
        let left = vertices[0].x.min(vertices[1].x).min(vertices[2].x);
        let right = vertices[0].x.max(vertices[1].x).max(vertices[2].x);
        let top = vertices[0].y.min(vertices[1].y).min(vertices[2].y);
        let bottom = vertices[0].y.max(vertices[1].y).max(vertices[2].y);

        let bounding_box = BoundingBox {
            left,
            right,
            top,
            bottom,
        };

        Self {
            vertices,
            bounding_box,
        }
    }

    pub fn area(&self) -> f32 {
        let (v1, v2) = (
            self.vertices[0] - self.vertices[1],
            self.vertices[2] - self.vertices[1],
        );
        (v1.cross(v2) * 0.5).abs()
    }

    pub fn render(&self, color: [u8; 3], rows: RowsMut<Rgb<u8>>) {
        let rendering_box = BoundingBox {
            left: self.bounding_box.left as usize,
            right: self.bounding_box.right as usize,
            top: self.bounding_box.top as usize,
            bottom: self.bounding_box.bottom as usize,
        };

        rows.rev()
            .skip(rendering_box.top)
            .take(rendering_box.height() + 1)
            .enumerate()
            .for_each(|(y, row)| {
                row.skip(rendering_box.left)
                    .take(rendering_box.width() + 1)
                    .enumerate()
                    .for_each(|(x, pixel)| {
                        if self.contains((x, y)) {
                            pixel.0 = color;
                        }
                    });
            });
    }

    fn contains(&self, (x, y): (usize, usize)) -> bool {
        let point = Vec2::new(
            (x + self.bounding_box.left as usize) as f32,
            (y + self.bounding_box.top as usize) as f32,
        );
        contains_triangle(
            (self.vertices[0], self.vertices[1], self.vertices[2]),
            point,
        )
    }
}

impl Segmentation {
    pub fn from_layer(layer: &UiLayer, vertices: &[(f32, f32)]) -> Self {
        let triangles = layer
            .triangles()
            .iter()
            .map(|triangle| Triangle::new(triangle, vertices))
            .collect();

        let color = layer.layer_info().color;
        let color = [
            (color[0] * 255.0) as u8,
            (color[1] * 255.0) as u8,
            (color[2] * 255.0) as u8,
        ];
        let vertices = layer
            .indices()
            .iter()
            .map(|&i| [vertices[i as usize].0, vertices[i as usize].1])
            .flatten()
            .collect();

        Self {
            id: layer.id(),
            type_id: layer.layer_info().id(),
            vertices,
            triangles,
            color,
        }
    }

    pub fn area(&self) -> f32 {
        self.triangles.iter().fold(0.0, |acc, tr| acc + tr.area())
    }

    pub fn bounding_box(&self) -> BoundingBox<f32> {
        let ultimate_resolution = 21370.0;
        let mut bounding_box =
            BoundingBox::<f32>::new(ultimate_resolution, ultimate_resolution, 0.0, 0.0);
        self.triangles.iter().for_each(|tr| {
            bounding_box.left = bounding_box.left.min(tr.bounding_box.left);
            bounding_box.right = bounding_box.right.max(tr.bounding_box.right);
            bounding_box.top = bounding_box.top.min(tr.bounding_box.top);
            bounding_box.bottom = bounding_box.bottom.max(tr.bounding_box.bottom);
        });
        bounding_box
    }

    pub fn vertices(&self) -> &Vec<f32> {
        &self.vertices
    }
}
