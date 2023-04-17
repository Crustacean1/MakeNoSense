use crate::vec::Vec2;

pub struct Intersector {
    vertices: Vec<Vec2>,
    segments: Vec<Segment<usize>>,
}

#[derive(Debug)]
pub struct Segment<T> {
    start: T,
    end: T,
}

impl Intersector {
    pub fn from_vertices(vertices: &Vec<Vec2>) -> Option<Self> {
        let vertices = vertices.clone();
        let segments = (0..vertices.len())
            .map(|i| Segment {
                start: i,
                end: (i + 1) % vertices.len(),
            })
            .collect();

        Some(Self { vertices, segments })
    }

    pub fn get_closed_loops(&mut self) -> Vec<Vec<Vec2>> {
        let cycles = vec![];

        self.resolve_all_intersections();

        while let Some(mut root) = self.find_root_segment() {
            let mut cycle = vec![];

            cycle.push(self.vertices[self.segments[root].start]);
            self.segments[root].start = 21370;
            self.segments[root].end = 21370;
            while let Some(segment) = self.find_next_segment(&self.segments[root]) {
                cycle.push(self.vertices[self.segments[segment].start]);
                self.segments[segment].start = 21370;
                self.segments[segment].end = 21370;
                root = segment;
            }
        }
        cycles
    }

    fn find_root_segment(&self) -> Option<usize> {
        Some(
            self.segments
                .iter()
                .enumerate()
                .find(|(i, s)| s.start != 21370)?
                .0,
        )
    }

    fn find_next_segment(&self, seg: &Segment<usize>) -> Option<usize> {
        Some(
            self.segments
                .iter()
                .enumerate()
                .find(|(i, s)| s.start == seg.end)?
                .0,
        )
    }

    fn resolve_all_intersections(&mut self) {
        while let Some(intersection) = self.find_intersection() {
            let s1 = &self.segments[intersection.0];
            let s2 = &self.segments[intersection.1];
        }
    }

    fn insert_missing_intersection(&mut self, a: &Segment<usize>, b: &Segment<usize>) {
        let segment_a = self.to_segment(a);
        let segment_b = self.to_segment(b);

        self.vertices
            .push(Self::create_intersection(&segment_a, &segment_b));
        self.vertices
            .push(Self::create_intersection(&segment_a, &segment_b));

        let intersection = (self.vertices.len() - 1, self.vertices.len() - 2);

        self.segments.remove(intersection.0);
        self.segments.remove(intersection.1);

        self.segments.push(Segment {
            start: a.start,
            end: intersection.0,
        });
        self.segments.push(Segment {
            start: intersection.0,
            end: b.end,
        });

        self.segments.push(Segment {
            start: b.start,
            end: intersection.1,
        });
        self.segments.push(Segment {
            start: intersection.1,
            end: a.end,
        });
    }

    fn find_intersection(&self) -> Option<(usize, usize)> {
        for i in 0..self.segments.len() {
            for j in i..self.segments.len() {
                let segment_a = self.to_segment(&self.segments[i]);
                let segment_b = self.to_segment(&self.segments[j]);

                if Self::segments_intersect(&segment_a, &segment_b) {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn to_segment(&self, segment: &Segment<usize>) -> Segment<Vec2> {
        Segment {
            start: self.vertices[segment.start],
            end: self.vertices[segment.end],
        }
    }

    fn segments_intersect(a: &Segment<Vec2>, b: &Segment<Vec2>) -> bool {
        let (a_norm, b_norm) = ((a.end - a.start).perp(), (b.end - b.start).perp());
        let (a_span, b_span) = (
            (a.end - b.start, a.start - b.start),
            (b.end - a.start, b.start - a.start),
        );

        (a_span.0 * b_norm) * (a_span.1 * b_norm) < -0.0
            && (b_span.0 * a_norm) * (b_span.1 * a_norm) < 0.0
    }

    fn create_intersection(a: &Segment<Vec2>, b: &Segment<Vec2>) -> Vec2 {
        let norm = (a.end - a.start).perp();
        let f_start = (b.end * norm).abs();
        let f_end = (b.start * norm).abs();
        let sum = f_start + f_end;

        Vec2::new((
            (f_start * b.start.x + f_end * b.end.x) / sum,
            (f_start * b.start.y + f_end * b.end.y) / sum,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn segments_should_intersect() {
        let segments = [
            (
                Segment {
                    start: Vec2::new((0.0, 0.0)),
                    end: Vec2::new((2.0, 0.0)),
                },
                Segment {
                    start: Vec2::new((1.0, -1.0)),
                    end: Vec2::new((1.0, 1.0)),
                },
            ),
            (
                Segment {
                    start: Vec2::new((0.0, 0.0)),
                    end: Vec2::new((1.0, 1.0)),
                },
                Segment {
                    start: Vec2::new((0.0, 1.0)),
                    end: Vec2::new((1.0, 0.0)),
                },
            ),
        ];
        for segment_pair in segments {
            assert_eq!(
                Intersector::segments_intersect(&segment_pair.0, &segment_pair.1),
                true,
            );
        }
    }

    #[test]
    fn segments_should_not_intersect() {
        let segments = [
            (
                Segment {
                    start: Vec2::new((0.0, 0.0)),
                    end: Vec2::new((2.0, 0.0)),
                },
                Segment {
                    start: Vec2::new((-1.0, 0.0)),
                    end: Vec2::new((-3.0, 0.0)),
                },
            ),
            (
                Segment {
                    start: Vec2::new((0.0, 0.0)),
                    end: Vec2::new((2.0, 0.0)),
                },
                Segment {
                    start: Vec2::new((1.0, 1.0)),
                    end: Vec2::new((3.0, 1.0)),
                },
            ),
        ];
        for segment_pair in segments {
            assert_eq!(
                Intersector::segments_intersect(&segment_pair.0, &segment_pair.1),
                false,
                "Fails: {:?}",
                segment_pair
            );
        }
    }

    #[test]
    fn intersection_point_lies_on_both_segments() {
        let segments = [
            (
                Segment {
                    start: Vec2::new((0.0, 0.0)),
                    end: Vec2::new((2.0, 0.0)),
                },
                Segment {
                    start: Vec2::new((1.0, -1.0)),
                    end: Vec2::new((1.0, 1.0)),
                },
            ),
            (
                Segment {
                    start: Vec2::new((0.0, 0.0)),
                    end: Vec2::new((1.0, 1.0)),
                },
                Segment {
                    start: Vec2::new((0.0, 1.0)),
                    end: Vec2::new((1.0, 0.0)),
                },
            ),
        ];
        let point = Intersector::create_intersection(&segments[0].0, &segments[0].1);
        assert_eq!(point.x, 1.0, "Invalid coordinate for: {:?}", segments[0]);
        assert_eq!(point.y, 0.0, "Invalid coordinate for: {:?}", segments[0]);

        let point = Intersector::create_intersection(&segments[1].0, &segments[1].1);
        assert_eq!(point.x, 0.5, "Invalid coordinate for: {:?}", segments[1]);
        assert_eq!(point.y, 0.5, "Invalid coordinate for: {:?}", segments[1]);
    }
}
