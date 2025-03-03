use super::{Drawable, Figure, PolygonTransform, Selectable};
use crate::lines;
use eframe::egui::{Color32, Painter, Pos2, Rect, Shape, Vec2};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

pub struct Polygon {
    control_points: Vec<Pos2>,
    inner_shapes: RefCell<Vec<Shape>>,
    normals: Vec<Vec2>,
    intercection_points: Vec<Pos2>,
    selected: bool,
    update_func: RefCell<Box<dyn Iterator<Item = Vec<Shape>>>>,
    update_delay: Option<std::time::Duration>,
    update_buffer: RefCell<VecDeque<Shape>>,
    last_update: RefCell<Option<std::time::Instant>>,
}

impl Polygon {
    pub fn new(control_points: Vec<Pos2>) -> Self {
        let new = Self {
            control_points,
            selected: false,
            normals: vec![],
            inner_shapes: RefCell::new(vec![]),
            intercection_points: vec![],
            update_func: RefCell::new(Box::new(std::iter::empty())),
            update_delay: Some(std::time::Duration::from_micros(100)),
            update_buffer: RefCell::new(VecDeque::new()),
            last_update: RefCell::new(None),
        };
        new
    }

    fn bounding_box(&self) -> (Pos2, Pos2) {
        self.control_points.iter().fold(
            (
                Pos2::new(f32::INFINITY, f32::INFINITY),
                Pos2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
            ),
            |(min, max), p| {
                (
                    Pos2::new(min.x.min(p.x), min.y.min(p.y)),
                    Pos2::new(max.x.max(p.x), max.y.max(p.y)),
                )
            },
        )
    }

    fn is_on_boundary(&self, p: Pos2, eps: f32) -> bool {
        self.control_points.windows(2).any(|pair| {
            let (a, b) = (pair[0], pair[1]);
            distance_to_line_segment(p, a, b) < eps
        })
    }

    // Проверка, внутри ли точка (метод чётности пересечений)
    fn is_inside(&self, point: Pos2) -> bool {
        let mut crossings = 0;
        let n = self.control_points.len();

        for i in 0..n {
            let p1 = self.control_points[i];
            let p2 = self.control_points[(i + 1) % n];

            if point.y > p1.y.min(p2.y) && point.y <= p1.y.max(p2.y) {
                let x_intersection = (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
                if point.x < x_intersection {
                    crossings += 1;
                }
            }
        }

        crossings % 2 != 0
    }

    fn find_intersections(&mut self, start: Pos2, end: Pos2) {
        let mut new_intersections = Vec::new();
        let n = self.control_points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let start_edge = self.control_points[i];
            let end_edge = self.control_points[j];
            if let Some(intersection) = intersect(start, end, start_edge, end_edge) {
                new_intersections.push(intersection);
            }
        }
        self.intercection_points.extend(new_intersections);
    }

    fn draw_circle(painter: &Painter, center: Pos2, color: (u8, u8, u8, u8)) {
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

                let color = Color32::from_rgba_premultiplied(
                    pixel_color.0,
                    pixel_color.1,
                    pixel_color.2,
                    pixel_color.3,
                );
                let rect = Rect::from_min_size(
                    Pos2 {
                        x: center.x + dx as f32,
                        y: center.y + dy as f32,
                    },
                    Vec2::new(1.0, 1.0),
                );
                painter.rect_filled(rect, 0.0, color);
            }
        }
    }

    const THICKNESS: f32 = 2.5;
    const THREASHOLD_HIT: f32 = 3.0;
}

impl Figure for Polygon {
    fn as_selectable(&self) -> Option<&dyn super::Selectable> {
        Some(self)
    }

    fn as_selectable_mut(&mut self) -> Option<&mut dyn Selectable> {
        Some(self)
    }

    fn as_polygon_transform(&self) -> Option<&dyn PolygonTransform> {
        Some(self)
    }

    fn as_polygon_transform_mut(&mut self) -> Option<&mut dyn PolygonTransform> {
        Some(self)
    }
}

impl Selectable for Polygon {
    fn select(&mut self) {
        self.selected = true;
    }

    fn deselect(&mut self) {
        self.selected = false;
    }

    fn hit_test(&self, pos: Pos2) -> bool {
        if self.control_points.is_empty() {
            return false;
        }

        let (min, max) = self.bounding_box();

        if pos.x < min.x || pos.x > max.x || pos.y < min.y || pos.y > max.y {
            return false;
        }

        if self.control_points.windows(2).any(|segment| {
            let p1 = segment[0];
            let p2 = segment[1];
            distance_to_line_segment(p1, p2, pos) <= Self::THREASHOLD_HIT
        }) {
            return true;
        }

        if self.control_points.len() > 1 {
            let start = self.control_points.last().unwrap();
            let end = self.control_points.first().unwrap();
            return distance_to_line_segment(*start, *end, pos) <= Self::THREASHOLD_HIT;
        }
        false
    }
}

impl Drawable for Polygon {
    fn draw(&self, painter: &eframe::egui::Painter) {
        if let Some(delay) = self.update_delay {
            let now = std::time::Instant::now();
            let mut last_update = self.last_update.borrow_mut();

            if let Some(last) = *last_update {
                let elapsed = now.duration_since(last);
                let count = (elapsed.as_secs_f64() / delay.as_secs_f64()).floor() as usize;

                if count > 0 {
                    *last_update = Some(last + delay * count as u32);
                    let mut buffer = self.update_buffer.borrow_mut();
                    let mut shapes = self.inner_shapes.borrow_mut();

                    for _ in 0..count {
                        if let Some(shape) = buffer.pop_front() {
                            shapes.push(shape);
                        } else {
                            if let Some(new_shapes) = self.update_func.borrow_mut().next() {
                                buffer.extend(new_shapes);
                            }
                            if let Some(shape) = buffer.pop_front() {
                                shapes.push(shape);
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    painter.ctx().request_repaint();
                }
            } else {
                *last_update = Some(now);
            }
        }
        draw_pixels(self.inner_shapes.borrow().clone(), &painter);

        for window in self.control_points.windows(2) {
            if let [start, end] = window {
                painter.line_segment(
                    [*start, *end],
                    eframe::egui::Stroke {
                        width: Self::THICKNESS,
                        color: if self.selected {
                            eframe::egui::Color32::from_white_alpha(255)
                        } else {
                            eframe::egui::Color32::from_black_alpha(255)
                        },
                    },
                );
            }
        }
        if self.control_points.len() > 1 {
            let start = self.control_points.last().unwrap();
            let end = self.control_points.first().unwrap();
            painter.line_segment(
                [*start, *end],
                eframe::egui::Stroke {
                    width: Self::THICKNESS,
                    color: if self.selected {
                        eframe::egui::Color32::from_white_alpha(255)
                    } else {
                        eframe::egui::Color32::from_black_alpha(255)
                    },
                },
            );
        }
        for pos in self.intercection_points.iter() {
            Self::draw_circle(painter, *pos, (255, 0, 0, 255));
        }
        if self.normals.len() == 0 {
            return;
        }
        let n = self.control_points.len();
        for i in 0..n {
            let a = self.control_points[i];
            let b = self.control_points[(i + 1) % n];

            // Calculate the midpoint of the edge (between a and b)
            let midpoint = Pos2::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);

            // Get the internal normal vector
            let normal = self.normals[i];

            // Scale the normal for better visibility
            let normal_end = midpoint + normal * 20.0; // Scale by 20 for better visibility

            // Draw the normal
            painter.line_segment(
                [midpoint, normal_end],
                eframe::egui::Stroke {
                    width: Self::THICKNESS,
                    color: eframe::egui::Color32::from_rgb(255, 0, 0), // Red color for normals
                },
            );
        }
    }
}

impl PolygonTransform for Polygon {
    fn test_convex(&self) -> bool {
        if self.control_points.len() < 3 {
            return false;
        }

        let mut last_sign: Option<bool> = None;
        let n = self.control_points.len();
        for i in 0..n {
            let o = self.control_points[i];
            let a = self.control_points[(i + 1) % n];
            let b = self.control_points[(i + 2) % n];

            let cp = cross_product(o, a, b);

            if cp != 0.0 {
                let current_sign = cp > 0.0;

                if last_sign.is_none() {
                    last_sign = Some(current_sign);
                } else if last_sign != Some(current_sign) {
                    return false;
                }
            }
        }

        true
    }

    fn find_internal_normals(&mut self) {
        let n = self.control_points.len();
        let mut normals = Vec::new();
        let polygon = &self.control_points;
        let center = find_centroid(polygon);

        for i in 0..n {
            let a = polygon[i];
            let b = polygon[(i + 1) % n];

            // Get the edge vector
            let edge = b - a;

            // Find the normal to this edge (perpendicular vector)
            let normal = Vec2::new(-edge.y, edge.x).normalized();

            // Check if the normal points towards the centroid (ensure inward direction)
            let to_centroid = Vec2::new(center.x - a.x, center.y - a.y);
            if normal.dot(to_centroid) < 0.0 {
                // If the normal does not point toward the centroid, invert it
                normals.push(Vec2::new(-normal.x, -normal.y));
            } else {
                normals.push(normal);
            }
        }
        self.normals = normals;
    }

    fn graham(&mut self) {
        if self.control_points.len() < 3 {
            return;
        }

        // Step 1: Find the point with the lowest y-coordinate (or leftmost if tie)
        let min_idx = self
            .control_points
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.y.partial_cmp(&b.y)
                    .unwrap()
                    .then_with(|| a.x.partial_cmp(&b.x).unwrap())
            })
            .map(|(idx, _)| idx)
            .unwrap();

        let pivot = self.control_points[min_idx];

        // Step 2: Sort the points by polar angle with respect to pivot
        let mut sorted_points = self.control_points.clone();
        sorted_points.sort_by(|a, b| {
            let cross = cross_product(pivot, *a, *b);
            if cross > 0.0 {
                std::cmp::Ordering::Less
            } else if cross < 0.0 {
                std::cmp::Ordering::Greater
            } else {
                // If collinear, sort by distance from pivot (closer points first)
                let dist_a = (a.x - pivot.x).hypot(a.y - pivot.y);
                let dist_b = (b.x - pivot.x).hypot(b.y - pivot.y);
                dist_a.partial_cmp(&dist_b).unwrap()
            }
        });

        // Step 3: Build the convex hull using a stack
        let mut hull = Vec::new();
        for &point in &sorted_points {
            while hull.len() >= 2 {
                let last = hull[hull.len() - 1];
                let second_last = hull[hull.len() - 2];

                if cross_product(second_last, last, point) <= 0.0 {
                    hull.pop(); // Remove last point if it's a right turn or collinear
                } else {
                    break;
                }
            }
            hull.push(point);
        }

        // Update the polygon to store only the convex hull
        self.control_points = hull;
    }

    fn jarvis(&mut self) {
        if self.control_points.len() < 3 {
            return;
        }

        let mut hull = Vec::new();
        let n = self.control_points.len();

        // Step 1: Find the leftmost point (smallest x, then smallest y)
        let mut leftmost_idx = 0;
        for i in 1..n {
            if self.control_points[i].x < self.control_points[leftmost_idx].x
                || (self.control_points[i].x == self.control_points[leftmost_idx].x
                    && self.control_points[i].y < self.control_points[leftmost_idx].y)
            {
                leftmost_idx = i;
            }
        }

        let mut current = leftmost_idx;

        loop {
            // Add the current point to the hull
            hull.push(self.control_points[current]);

            // Step 2: Find the most counterclockwise point
            let mut next = (current + 1) % n;
            for i in 0..n {
                let cross = cross_product(
                    self.control_points[current],
                    self.control_points[next],
                    self.control_points[i],
                );

                // If `i` is more counterclockwise, update `next`
                if cross > 0.0 // Changed from < to >
    || (cross == 0.0
        && (self.control_points[i].x - self.control_points[current].x)
            .hypot(self.control_points[i].y - self.control_points[current].y)
            > (self.control_points[next].x - self.control_points[current].x).hypot(
                self.control_points[next].y - self.control_points[current].y,
            )) {
                    next = i;
                }
            }

            // Stop when we wrap around to the first point
            if next == leftmost_idx {
                break;
            }
            current = next;
        }

        // Update the polygon to store only the convex hull
        self.control_points = hull;
    }

    fn test_dot(&self, point: Pos2) -> bool {
        self.is_inside(point)
    }

    fn test_line(&mut self, start: Pos2, end: Pos2) {
        {
            let mut inner_shapes = self.inner_shapes.borrow_mut();

            inner_shapes.extend(lines::dda_line(start, end).flat_map(|vec| {
                vec.into_iter().map(|pixel| {
                    Shape::rect_filled(
                        Rect::from_min_size(pixel.pos, Vec2::new(1.0, 1.0)),
                        0.0,
                        Color32::BLACK,
                    )
                })
            }));
        }
        self.find_intersections(start, end);
    }

    fn reset_fill(&mut self) {
        self.inner_shapes.borrow_mut().clear();
        self.update_buffer.borrow_mut().clear();
        self.update_func = RefCell::new(Box::new(std::iter::empty()));
    }

    fn first(&mut self) {
        if self.control_points.len() < 3 {
            return;
        }

        let mut edges: Vec<Edge> = Vec::new();
        let n = self.control_points.len();

        // Creating edges
        for i in 0..n {
            let p1 = self.control_points[i];
            let p2 = self.control_points[(i + 1) % n];

            if p1.y == p2.y {
                continue; // Ignore horizontal edges
            }

            let (p1, p2) = if p1.y < p2.y { (p1, p2) } else { (p2, p1) };
            let slope_inverse = (p2.x - p1.x) / (p2.y - p1.y);

            edges.push(Edge {
                y_max: p2.y,
                y_min: p1.y,
                x_min: p1.x,
                slope_inverse,
            });
        }

        // Sort edges by x_min
        edges.sort_by(|a, b| {
            a.x_min
                .partial_cmp(&b.x_min)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut y = self
            .control_points
            .iter()
            .map(|p| p.y)
            .fold(f32::INFINITY, f32::min); // Find min y
        let y_max = self
            .control_points
            .iter()
            .map(|p| p.y)
            .fold(f32::NEG_INFINITY, f32::max); // Find max y

        let func_iter = std::iter::from_fn(move || {
            if y <= y_max {
                let mut buffer = Vec::new();
                // Check which edges intersect the current scanline (y)
                let mut intersections = Vec::new();

                // Find edges that are active at this y
                for edge in edges.iter() {
                    if edge.y_min <= y && edge.y_max > y {
                        let x_intersection =
                            edge.x_min + (y - edge.y_min) as f32 * edge.slope_inverse;
                        intersections.push(x_intersection);
                    }
                }

                intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

                for i in (0..intersections.len()).step_by(2) {
                    if i + 1 < intersections.len() {
                        let x_start = intersections[i].round() as u32;
                        let x_end = intersections[i + 1].round() as u32;

                        for x in x_start..=x_end {
                            buffer.push(get_rect_shape(x as f32, y.floor()));
                            // self.inner_shapes.push(get_rect_shape(x as f32, y.floor()));
                        }
                    }
                }

                y += 1.0;
                Some(buffer)
            } else {
                None
            }
        });

        self.update_func = RefCell::new(Box::new(func_iter));
    }

    fn second(&mut self) {
        if self.control_points.len() < 3 {
            return;
        }

        let mut edges: Vec<Edge> = Vec::new();
        let n = self.control_points.len();

        // Creating edges
        for i in 0..n {
            let p1 = self.control_points[i];
            let p2 = self.control_points[(i + 1) % n];

            if p1.y == p2.y {
                continue; // Ignore horizontal edges
            }

            let (p1, p2) = if p1.y < p2.y { (p1, p2) } else { (p2, p1) };
            let slope_inverse = (p2.x - p1.x) / (p2.y - p1.y);

            edges.push(Edge {
                y_max: p2.y,
                y_min: p1.y,
                x_min: p1.x,
                slope_inverse,
            });
        }

        // Sort edges by y_min, then by x_min
        edges.sort_by(|a, b| {
            a.y_min
                .partial_cmp(&b.y_min)
                .unwrap()
                .then(a.x_min.partial_cmp(&b.x_min).unwrap())
        });

        let mut y = edges.first().map(|e| e.y_min).unwrap_or(0.0);
        let y_max = edges
            .iter()
            .map(|e| e.y_max)
            .fold(f32::NEG_INFINITY, f32::max);
        let mut active_edges: Vec<Edge> = Vec::new();

        // Filling each scanline
        let func_iter = std::iter::from_fn(move || {
            if y <= y_max {
                let mut buffer = Vec::new();
                // Move edges from global edge list to active edge list
                edges.retain(|edge| {
                    if edge.y_min as u32 == y as u32 {
                        active_edges.push(edge.clone());
                        false
                    } else {
                        true
                    }
                });

                // Remove edges where y reaches y_max
                active_edges.retain(|edge| edge.y_max as u32 > y as u32);

                // Sort active edges by x_min
                active_edges.sort_by(|a, b| a.x_min.partial_cmp(&b.x_min).unwrap());

                let intersections: Vec<f32> = active_edges.iter().map(|edge| edge.x_min).collect();

                for i in (0..intersections.len()).step_by(2) {
                    if i + 1 < intersections.len() {
                        let x_start = intersections[i].round() as u32;
                        let x_end = intersections[i + 1].round() as u32;

                        for x in x_start..=x_end {
                            buffer.push(get_rect_shape(x as f32, y.floor()));
                            // self.inner_shapes.push(get_rect_shape(x as f32, y.floor()));
                        }
                    }
                }

                // Update x_min for next scanline
                for edge in active_edges.iter_mut() {
                    edge.x_min += edge.slope_inverse;
                }

                y += 1.0;
                Some(buffer)
            } else {
                None
            }
        });

        self.update_func = RefCell::new(Box::new(func_iter));
    }

    fn third(&mut self) {
        let mut visited = HashSet::new();
        let step = 0.000001;
        let start = find_centroid(&self.control_points);
        let sx = start.x as i32;
        let sy = start.y as i32;
        let mut stack = vec![(sx, sy)];
        let polygon = self.control_points.clone();
        let func_iter = std::iter::from_fn(move || {
            loop {
                if let Some((x, y)) = stack.pop() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }

                    let p = Pos2 {
                        x: x as f32,
                        y: y as f32,
                    };
                    if is_on_boundary(&polygon, p, step / 2.0) || !is_inside(&polygon, p) {
                        continue;
                    }

                    visited.insert((x, y));
                    // self.inner_shapes.push(get_rect_shape(p.x, p.y));

                    stack.push((x + 1, y));
                    stack.push((x - 1, y));
                    stack.push((x, y + 1));
                    stack.push((x, y - 1));
                    return Some(vec![get_rect_shape(p.x, p.y)]);
                } else {
                    return None;
                }
            }
        });

        self.update_func = RefCell::new(Box::new(func_iter));
    }

    fn fourth(&mut self) {
        let step = 0.000001;
        let start = find_centroid(&self.control_points);
        let sx = start.x as i32;
        let sy = start.y as i32;
        let mut stack = vec![(sx, sy)];
        let mut visited = HashSet::new();
        let polygon = self.control_points.clone();
        let func_iter = std::iter::from_fn(move || {
            loop {
                if let Some((x, y)) = stack.pop() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }

                    let p = Pos2 {
                        x: x as f32,
                        y: y as f32,
                    };

                    if is_on_boundary(&polygon, p, step / 2.0) || !is_inside(&polygon, p) {
                        continue;
                    }
                    let mut buffer = vec![];

                    visited.insert((x, y));
                    buffer.push(get_rect_shape(p.x, p.y));
                    // self.inner_shapes.push(get_rect_shape(p.x, p.y));

                    // Fill left and right
                    let mut left = x;
                    while !is_on_boundary(
                        &polygon,
                        Pos2 {
                            x: (left - 1) as f32,
                            y: y as f32,
                        },
                        step / 2.0,
                    ) && is_inside(
                        &polygon,
                        Pos2 {
                            x: (left - 1) as f32,
                            y: y as f32,
                        },
                    ) {
                        left -= 1;
                        if !visited.contains(&(left, y)) {
                            visited.insert((left, y));
                            buffer.push(get_rect_shape(left as f32, y as f32));
                            // self.inner_shapes
                            // .push(get_rect_shape(left as f32, y as f32));
                        }
                    }

                    let mut right = x;
                    while !is_on_boundary(
                        &polygon,
                        Pos2 {
                            x: (right + 1) as f32,
                            y: y as f32,
                        },
                        step / 2.0,
                    ) && is_inside(
                        &polygon,
                        Pos2 {
                            x: (right + 1) as f32,
                            y: y as f32,
                        },
                    ) {
                        right += 1;
                        if !visited.contains(&(right, y)) {
                            visited.insert((right, y));
                            buffer.push(get_rect_shape(right as f32, y as f32))
                            // self.inner_shapes
                            //     .push(get_rect_shape(right as f32, y as f32));
                        }
                    }

                    // Check upper and lower rows for new seeds
                    let mut add_to_stack = |x, y| {
                        // Check if the current pixel is within bounds and unvisited
                        let p_check = Pos2 {
                            x: x as f32,
                            y: y as f32,
                        };
                        if !visited.contains(&(x, y)) && is_inside(&polygon, p_check) {
                            stack.push((x, y));
                        }
                    };

                    // Check the region around the filled row to find unfilled pixels
                    for dy in (-1..=1).step_by(2) {
                        // Check one row above, one below, and the current row
                        for dx in left..=right {
                            add_to_stack(dx, y + dy);
                        }
                    }
                    return Some(buffer);
                } else {
                    return None;
                }
            }
        });
        self.update_func = RefCell::new(Box::new(func_iter));
    }
}

#[derive(Clone, Debug)]
struct Edge {
    y_max: f32,
    y_min: f32,
    x_min: f32,
    slope_inverse: f32,
}

fn distance_to_line_segment(p1: Pos2, p2: Pos2, point: Pos2) -> f32 {
    let v = p2 - p1;
    let u = point - p1;
    let t = (u.x * v.x + u.y * v.y) / (v.x * v.x + v.y * v.y);
    let t_clamped = t.clamp(0.0, 1.0);
    let closest = Pos2::new(p1.x + t_clamped * v.x, p1.y + t_clamped * v.y);
    point.distance(closest)
}

fn get_rect_shape(x: f32, y: f32) -> Shape {
    Shape::rect_filled(
        Rect::from_min_size(Pos2::new(x, y), Vec2::new(1.0, 1.0)),
        0.0,
        Color32::BLACK,
    )
}

fn draw_pixels(shapes: Vec<Shape>, painter: &Painter) {
    painter.extend(shapes);
}

fn intersect(start: Pos2, end: Pos2, start_edge: Pos2, end_edge: Pos2) -> Option<Pos2> {
    let denom = (start.x - end.x) * (start_edge.y - end_edge.y)
        - (start.y - end.y) * (start_edge.x - end_edge.x);
    if denom.abs() < 1e-9 {
        return None;
    }

    let t = ((start.x - start_edge.x) * (start_edge.y - end_edge.y)
        - (start.y - start_edge.y) * (start_edge.x - end_edge.x))
        / denom;
    let u = ((start.x - start_edge.x) * (start.y - end.y)
        - (start.y - start_edge.y) * (start.x - end.x))
        / denom;

    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        let px = start.x + t * (end.x - start.x);
        let py = start.y + t * (end.y - start.y);
        return Some(Pos2::new(px, py));
    } else {
        return None;
    }
}

fn cross_product(o: Pos2, a: Pos2, b: Pos2) -> f32 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn find_centroid(polygon: &[Pos2]) -> Pos2 {
    let n = polygon.len();
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    for point in polygon {
        sum_x += point.x;
        sum_y += point.y;
    }

    Pos2::new(sum_x / n as f32, sum_y / n as f32)
}

fn is_on_boundary(polygon: &[Pos2], p: Pos2, eps: f32) -> bool {
    polygon.windows(2).any(|pair| {
        let (a, b) = (pair[0], pair[1]);
        distance_to_line_segment(p, a, b) < eps
    })
}

fn is_inside(polygon: &[Pos2], point: Pos2) -> bool {
    let mut crossings = 0;
    let n = polygon.len();

    for i in 0..n {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % n];

        if point.y > p1.y.min(p2.y) && point.y <= p1.y.max(p2.y) {
            let x_intersection = (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;
            if point.x < x_intersection {
                crossings += 1;
            }
        }
    }

    crossings % 2 != 0
}

// // Функция для вычисления расстояния от точки до отрезка
// fn distance_point_to_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
//     let ab = Pos2 {
//         x: b.x - a.x,
//         y: b.y - a.y,
//     };
//     let ap = Pos2 {
//         x: p.x - a.x,
//         y: p.y - a.y,
//     };
//     let bp = Pos2 {
//         x: p.x - b.x,
//         y: p.y - b.y,
//     };
//
//     let dot1 = ab.x * ap.x + ab.y * ap.y;
//     let dot2 = ab.x * bp.x + ab.y * bp.y;
//
//     if dot1 <= 0.0 {
//         return ((p.x - a.x).powi(2) + (p.y - a.y).powi(2)).sqrt();
//     }
//     if dot2 >= 0.0 {
//         return ((p.x - b.x).powi(2) + (p.y - b.y).powi(2)).sqrt();
//     }
//
//     let cross = ab.x * ap.y - ab.y * ap.x;
//     (cross.abs() / ((b.x - a.x).hypot(b.y - a.y))).abs()
// }
//
