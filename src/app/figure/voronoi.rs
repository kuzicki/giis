use super::{Delone, Drawable, Figure, Triangle};
use eframe::egui::{Pos2, Vec2};

pub struct Voronoi {
    lines: Vec<(Pos2, Pos2)>,
}

impl Triangle {
    fn get_center_point(&self) -> Pos2 {
        let ax = self.a.x;
        let ay = self.a.y;
        let bx = self.b.x;
        let by = self.b.y;
        let cx = self.c.x;
        let cy = self.c.y;

        let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
        if d == 0.0 {
            return Pos2 {
                x: f32::NAN,
                y: f32::NAN,
            };
        }

        let ux = ((ax * ax + ay * ay) * (by - cy)
            + (bx * bx + by * by) * (cy - ay)
            + (cx * cx + cy * cy) * (ay - by))
            / d;
        let uy = ((ax * ax + ay * ay) * (cx - bx)
            + (bx * bx + by * by) * (ax - cx)
            + (cx * cx + cy * cy) * (bx - ax))
            / d;

        Pos2 { x: ux, y: uy }
    }

    fn get_third_vertex(&self, point1: Pos2, point2: Pos2) -> Pos2 {
        if self.a != point1 && self.a != point2 {
            self.a
        } else if self.b != point1 && self.b != point2 {
            self.b
        } else {
            self.c
        }
    }
}

impl Voronoi {
    pub fn new(points: Vec<Pos2>) -> Self {
        let delone = Delone::new(points);
        let triangles = delone.get_triangles();
        let mut lines = vec![];
        for (ind, t) in triangles.iter().enumerate() {
            for (edge_ind, edge) in t.get_edges().iter().enumerate() {
                let mut found_adjacent = false;
                for (ind_other, t_other) in triangles.iter().enumerate() {
                    if ind == ind_other {
                        continue;
                    }

                    for edge_other in t_other.get_edges() {
                        if edge.equal(&edge_other) {
                            lines.push((t.get_center_point(), t_other.get_center_point()));
                            found_adjacent = true;
                            break;
                        }
                    }
                    if found_adjacent {
                        break;
                    }
                }
                if found_adjacent {
                    continue;
                }
                let edge_vector = edge.b - edge.a;
                let midpoint = Pos2::new((edge.a.x + edge.b.x) * 0.5, (edge.a.y + edge.b.y) * 0.5);
                if edge_vector.length_sq() > 0.0 {
                    let third_vertex = t.get_third_vertex(edge.a, edge.b);
                    let perp = Vec2::new(-edge_vector.y, edge_vector.x).normalized();

                    let to_third = third_vertex - midpoint;
                    let perp = if perp.dot(to_third) > 0.0 {
                        -perp
                    } else {
                        perp
                    };

                    let length = 10000.0;
                    let start = t.get_center_point();
                    let end = start + perp * length;
                    lines.push((start, end));
                }
            }
        }

        Self {
            lines,
        }
    }
}

impl Figure for Voronoi {}

impl Drawable for Voronoi {
    fn draw(&self, painter: &eframe::egui::Painter) {
        for (a, b) in self.lines.iter() {
            painter.add(eframe::egui::Shape::line_segment(
                [*a, *b],
                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::RED),
            ));
        }
    }
}
