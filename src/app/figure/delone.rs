use super::{draw_pixels, Drawable, Figure};
use crate::pixel::Pixel;
use eframe::egui::Pos2;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq)]
struct Triangle {
    a: Pos2,
    b: Pos2,
    c: Pos2,
}

#[derive(Clone)]
struct Edge {
    a: Pos2,
    b: Pos2,
}

impl Edge {
    fn new(mut a: Pos2, mut b: Pos2) -> Self {
        if a.x > b.x || (a.x == b.x && a.y > b.y) {
            std::mem::swap(&mut a, &mut b);
        }
        Self { a, b }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.x.to_bits().hash(state);
        self.a.y.to_bits().hash(state);
        self.b.x.to_bits().hash(state);
        self.b.y.to_bits().hash(state);
    }
}

impl Triangle {
    fn get_edges(&self) -> [Edge; 3] {
        [
            Edge::new(self.a, self.b),
            Edge::new(self.b, self.c),
            Edge::new(self.c, self.a),
        ]
    }

    fn is_point_inside(&self, point: Pos2) -> bool {
        Self::in_circumcircle(self.a.clone(), self.b.clone(), self.c.clone(), point)
    }

    fn is_counterclockwise(a: &Pos2, b: &Pos2, c: &Pos2) -> bool {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x) > 0.0
    }

    fn in_circumcircle(a: Pos2, mut b: Pos2, mut c: Pos2, d: Pos2) -> bool {
        if !Self::is_counterclockwise(&a, &b, &c) {
            std::mem::swap(&mut b, &mut c);
        }

        let adx = a.x - d.x;
        let ady = a.y - d.y;
        let bdx = b.x - d.x;
        let bdy = b.y - d.y;
        let cdx = c.x - d.x;
        let cdy = c.y - d.y;

        let det = (adx * adx + ady * ady) * (bdx * cdy - bdy * cdx)
            - (bdx * bdx + bdy * bdy) * (adx * cdy - ady * cdx)
            + (cdx * cdx + cdy * cdy) * (adx * bdy - ady * bdx);

        det > 0.0
    }
}

pub struct Delone {
    triangles: Vec<Triangle>,
}

impl Delone {
    pub fn new(points: Vec<Pos2>) -> Self {
        let mut delone = Self { triangles: vec![] };
        let super_triangle = Triangle {
            a: Pos2 { x: -300.0, y: 0.0 },
            b: Pos2 { x: 1000.0, y: 0.0 },
            c: Pos2 {
                x: 650.0,
                y: 2000.0,
            },
        };
        delone.triangles.push(super_triangle);
        for point in points {
            delone.add_point(point);
        }
        let mut bad_triangles = vec![];
        for t in &delone.triangles {
            'outer: for super_edges in &super_triangle.get_edges() {
                for edges in t.get_edges() {
                    if super_edges.a == edges.a
                        || super_edges.a == edges.b
                        || super_edges.b == edges.a
                        || super_edges.b == edges.b
                    {
                        println!("Broke");
                        bad_triangles.push(*t);
                        break 'outer;
                    }
                }
            }
        }

        delone
            .triangles
            .retain(|t| bad_triangles.iter().all(|t_other| t != t_other));

        delone
    }

    fn add_point(&mut self, point: Pos2) {
        let mut bad_triangle_edges = HashMap::new();
        let mut bad_triangles = vec![];
        let insert_or_increment = |map: &mut HashMap<Edge, i32>, edge: Edge| {
            *map.entry(edge).or_insert(0) += 1;
        };

        for t in self.triangles.iter() {
            if t.is_point_inside(point) {
                bad_triangles.push(*t);
                for edge in t.get_edges() {
                    insert_or_increment(&mut bad_triangle_edges, edge);
                }
            }
        }
        let polygon: Vec<_> = bad_triangle_edges
            .iter()
            .filter(|&(_, &times)| times == 1)
            .map(|(edge, _)| edge)
            .collect();
        self.triangles
            .retain(|t| bad_triangles.iter().all(|t_other| t != t_other));

        for edge in polygon {
            self.triangles.push(Triangle {
                a: edge.a,
                b: edge.b,
                c: point,
            });
        }
    }
}

impl Figure for Delone {}

impl Drawable for Delone {
    fn draw(&self, painter: &eframe::egui::Painter) {
        for triangle in &self.triangles {
            // Draw the edges of the triangle by connecting the three points a, b, and c
            painter.add(eframe::egui::Shape::line_segment(
                [triangle.a, triangle.b],
                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::WHITE),
            ));
            painter.add(eframe::egui::Shape::line_segment(
                [triangle.b, triangle.c],
                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::WHITE),
            ));
            painter.add(eframe::egui::Shape::line_segment(
                [triangle.c, triangle.a],
                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::WHITE),
            ));
        }
    }
}
