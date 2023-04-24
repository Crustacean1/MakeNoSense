use std::{iter::zip, ops};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn cross(self, rhs: Vec2) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn perp(&self) -> Vec2 {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul for Vec2 {
    type Output = f32;
    fn mul(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

pub fn triangulate(vertices: &Vec<(f32, f32)>, indices: &Vec<u32>, triangles: &mut Vec<[u32; 3]>) {
    triangles.clear();
    let points = indices
        .iter()
        .map(|&i| Vec2::new(vertices[i as usize].0, vertices[i as usize].1))
        .collect();
    triangulate_convex(&points, &indices, triangles);
}

fn triangulate_convex(
    points: &Vec<Vec2>,
    indices: &Vec<u32>,
    triangles: &mut Vec<[u32; 3]>,
) -> Option<()> {
    if points.len() < 3 {
        return None;
    }

    let winding = compute_winding(points)?;

    let mut ears = vec![];
    let mut points_left = points.clone();

    while points_left.len() >= 3 {
        let point = (0..points_left.len()).find(|point| {
            let triangle = get_triangle(&points_left, *point);
            let (s1, s2) = (triangle.0 - triangle.1, triangle.2 - triangle.1);
            let cross = s1.cross(s2);
            if cross * winding < 0.0 {
                !points_left
                    .iter()
                    .any(|point| triangle_contains(triangle, *point))
            } else {
                false
            }
        });
        match point {
            Some(point) => {
                ears.push(point);
                points_left.remove(point);
            }
            None => {
                println!("Fucksie wucksie");
                break;
            }
        }
    }

    let mut mesh_indices: Vec<_> = indices.clone();

    ears.iter().for_each(|i| {
        let (p1, p2, p3) = get_triangle(&mesh_indices, *i);
        triangles.push([p1 as u32, p2 as u32, p3 as u32]);
        mesh_indices.remove(*i);
    });

    Some(())
}

fn compute_winding(points: &Vec<Vec2>) -> Option<f32> {
    let origin = *points.first()?;

    let winding =
        zip(points.iter().skip(1), points.iter().skip(2)).fold(0.0, |winding, (&v1, &v2)| {
            let v1 = v1 - origin;
            let v2 = v2 - origin;
            winding + v1.cross(v2)
        });
    Some(winding)
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

fn get_triangle<T: Copy + std::fmt::Debug>(vertices: &Vec<T>, i: usize) -> (T, T, T) {
    let length = vertices.len();
    (
        vertices[(i + length - 1) % length],
        vertices[i],
        vertices[(i + 1) % length],
    )
}
