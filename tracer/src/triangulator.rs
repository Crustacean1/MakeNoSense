use std::iter::zip;

use crate::{intersector::Intersector, vec::Vec2};

pub struct Triangulator {
    points: Vec<Vec2>,
    vertices: Vec<Vec2>,
    indices: Vec<[u32; 3]>,
}

impl Triangulator {
    pub fn new() -> Self {
        Triangulator {
            points: vec![Vec2::new((0.0, 0.0))],
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn add(&mut self, point: Vec2) -> Option<()> {
        *self.points.last_mut()? = point;
        self.points.push(point);
        Some(())
    }

    pub fn update(&mut self, pos: Vec2) -> Option<()> {
        Some(*self.points.last_mut()? = pos)
    }

    fn compute_winding(&mut self) -> Option<f32> {
        let origin = *self.points.first()?;

        let winding = zip(self.points.iter().skip(1), self.points.iter().skip(2)).fold(
            0.0,
            |winding, (&v1, &v2)| {
                let v1 = v1 - origin;
                let v2 = v2 - origin;
                winding + v1.cross(v2)
            },
        );
        Some(winding)
    }

    pub fn triangulate(&mut self) -> Option<(&Vec<Vec2>, Vec<[u32; 3]>)> {
        self.vertices.clear();
        self.indices.clear();

        //let intersector = Intersector::from_vertices(&self.points);
        self.triangulate_convex()?;
        self.vertices = self.points.clone();
        Some((&self.vertices, self.indices.clone()))
    }

    pub fn get_points(&self) -> &Vec<Vec2> {
        &self.points
    }

    pub fn triangulate_convex(&mut self) -> Option<()> {
        if self.points.len() < 3 {
            return None;
        }

        let winding = self.compute_winding()?;

        let mut ears = vec![];
        let mut points = self.points.clone();

        while points.len() >= 3 {
            let point = (0..points.len()).find(|point| {
                let triangle = Self::get_triangle(&points, *point);
                let (s1, s2) = (triangle.0 - triangle.1, triangle.2 - triangle.1);
                let cross = s1.cross(s2);
                if cross * winding < 0.0 {
                    !points
                        .iter()
                        .any(|point| Self::triangle_contains(triangle, *point))
                } else {
                    false
                }
            });
            match point {
                Some(point) => {
                    ears.push(point);
                    points.remove(point);
                }
                None => {
                    break;
                }
            }
        }

        let mut mesh_indices: Vec<_> = (0..self.points.len()).collect();

        ears.iter().for_each(|i| {
            let (p1, p2, p3) = Self::get_triangle(&mesh_indices, *i);
            self.indices.push([p1 as u32, p2 as u32, p3 as u32]);
            mesh_indices.remove(*i);
        });

        Some(())
    }

    fn triangle_contains(triangle: (Vec2, Vec2, Vec2), point: Vec2) -> bool {
        let normals = (
            (triangle.0 - triangle.1).perp(),
            (triangle.1 - triangle.2).perp(),
            (triangle.2 - triangle.0).perp(),
        );
        let signs = (
            (point - triangle.0) * normals.0,
            (point - triangle.1) * normals.1,
            (point - triangle.2) * normals.2,
        );
        (signs.0 < 0.0 && signs.1 < 0.0 && signs.2 < 0.0)
            || (signs.0 > 0.0 && signs.1 > 0.0 && signs.2 > 0.0)
    }

    fn get_triangle<T: Copy>(vertices: &Vec<T>, i: usize) -> (T, T, T) {
        let length = vertices.len();
        (
            vertices[(i + length - 1) % length],
            vertices[i],
            vertices[(i + 1) % length],
        )
    }
}
