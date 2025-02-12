use super::{ConnectableCurves, EditableControlPoints, Figure, Selectable};
use crate::curves::{generate_bezier_curve, generate_bspline_curve, generate_hermite_curve};
use crate::pixel::Pixel;
use eframe::egui::Pos2;

#[derive(Clone)]
pub enum CurveType {
    Hermite,
    Bezier,
    BSpline,
}

pub struct Curve {
    points: Vec<Pixel>,
    control_points: [Pos2; 4],
    selected: bool,
    curve_type: CurveType,
    selected_point: Option<usize>,
}

impl Curve {
    pub fn new(control_points: [Pos2; 4], curve_type: CurveType) -> Self {
        let mut new = Self {
            control_points,
            points: vec![],
            selected: false,
            selected_point: None,
            curve_type,
        };
        new.update_render();
        new
    }

    fn update_render(&mut self) {
        self.points.clear();
        self.generate_curve();

        if self.selected {
            self.draw_control_points();
        }
    }

    fn draw_control_points(&mut self) {
        for (index, pos) in self.control_points.iter().enumerate() {
            let is_selected = self.selected_point == Some(index);
            let color = if is_selected {
                (50, 50, 50, 255)
            } else {
                (255, 0, 0, 255)
            };
            Self::draw_circle(&mut self.points, *pos, color);
        }
    }

    fn draw_circle(points: &mut Vec<Pixel>, center: Pos2, color: (u8, u8, u8, u8)) {
        for dx in -Self::THICKNESS as i32..=Self::THICKNESS as i32 {
            for dy in -Self::THICKNESS as i32..=Self::THICKNESS as i32 {
                let distance = (dx as f32).hypot(dy as f32);

                let pixel_color =
                    if distance >= Self::THICKNESS - 1.0 && distance <= Self::THICKNESS {
                        let alpha = (1.0 - (distance - (Self::THICKNESS - 1.0)) / 2.0).max(0.0);
                        (color.0, color.1, color.2, (alpha * 255.0) as u8)
                    } else if distance < Self::THICKNESS - 1.0 {
                        color
                    } else {
                        continue;
                    };

                points.push(Pixel::new_pos2(
                    Pos2 {
                        x: center.x + dx as f32,
                        y: center.y + dy as f32,
                    },
                    pixel_color,
                ));
            }
        }
    }

    fn generate_curve(&mut self) {
        match self.curve_type {
            CurveType::Bezier => generate_bezier_curve(
                self.control_points[0],
                self.control_points[1],
                self.control_points[2],
                self.control_points[3],
                &mut self.points,
            ),
            CurveType::Hermite => generate_hermite_curve(
                self.control_points[0],
                self.control_points[1],
                self.control_points[2],
                self.control_points[3],
                &mut self.points,
            ),
            CurveType::BSpline => {
                generate_bspline_curve(&self.control_points, &mut self.points)
            }
        }
    }

    const THICKNESS: f32 = 2.5;
    const THREASHOLD_HIT: f32 = 3.0;
}

impl Figure for Curve {
    fn get_pixels(&self) -> &[Pixel] {
        self.points.as_slice()
    }

    fn as_selectable(&self) -> Option<&dyn super::Selectable> {
        Some(self)
    }

    fn as_selectable_mut(&mut self) -> Option<&mut dyn Selectable> {
        Some(self)
    }

    fn as_editable_points(&self) -> Option<&dyn EditableControlPoints> {
        Some(self)
    }

    fn as_editable_points_mut(&mut self) -> Option<&mut dyn EditableControlPoints> {
        Some(self)
    }
}

impl Selectable for Curve {
    fn select(&mut self) {
        self.selected = true;
        self.update_render();
    }

    fn deselect(&mut self) {
        self.selected = false;
        self.selected_point = None;
        self.update_render();
    }


    fn hit_test(&self, pos: Pos2) -> bool {
        if self.points.is_empty() {
            return false;
        }

        let (min, max) = self.bounding_box();

        // Step 2: Quick Bounding Box Rejection
        if pos.x < min.x || pos.x > max.x || pos.y < min.y || pos.y > max.y
        {
            return false;
        }

        // Step 3: Check Distance to Line Segments
        self.points.windows(2).any(|segment| {
            let p1 = segment[0].pos;
            let p2 = segment[1].pos;
            distance_to_line_segment(p1, p2, pos) <= Self::THREASHOLD_HIT
        })
    }

    fn bounding_box(&self) -> (Pos2, Pos2) {
        self.points.iter().fold(
            (
                Pos2::new(f32::INFINITY, f32::INFINITY),
                Pos2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
            ),
            |(min, max), p| {
                (
                    Pos2::new(min.x.min(p.pos.x), min.y.min(p.pos.y)),
                    Pos2::new(max.x.max(p.pos.x), max.y.max(p.pos.y)),
                )
            },
        )
    }
}

impl EditableControlPoints for Curve {
    fn control_points(&self) -> &[Pos2] {
        self.control_points.as_slice()
    }

    fn control_points_mut(&mut self) -> &mut [Pos2] {
        self.control_points.as_mut_slice()
    }

    fn hit_test_control_point(&self, pos: Pos2, radius: f32) -> Option<usize> {
        self.control_points.iter().enumerate().find_map(|(i, &p)| {
            if p.distance(pos) <= radius {
                Some(i)
            } else {
                None
            }
        })
    }

    fn toggle_point(&mut self, index: usize) {
        self.selected_point = match self.selected_point {
            Some(selected) => {
                if selected == index {
                    None
                } else {
                    Some(index)
                }
            }
            None => Some(index),
        };
        self.update_render();
    }

    fn move_point(&mut self, pos: Pos2) -> bool {
        if let Some(index) = self.selected_point {
            self.control_points[index] = pos;
            self.update_render();
            return true;
        }
        false
    }
}

impl ConnectableCurves for Curve {
    fn connect_curves(&mut self, other: &mut Self) {
        unimplemented!()
    }
}

fn distance_to_line_segment(p1: Pos2, p2: Pos2, point: Pos2) -> f32 {
    let v = p2 - p1;
    let u = point - p1;
    let t = (u.x * v.x + u.y * v.y) / (v.x * v.x + v.y * v.y);
    let t_clamped = t.clamp(0.0, 1.0);
    let closest = Pos2::new(p1.x + t_clamped * v.x, p1.y + t_clamped * v.y);
    point.distance(closest)
}
