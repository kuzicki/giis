use crate::pixel::Pixel;
use eframe::egui;

pub fn paint_circle(
    start: egui::Pos2,
    end: egui::Pos2,
) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
    let mut x = 0;
    let r = start.distance(end) as i32;
    let mut y = r;
    let mut d = 2 - 2 * r;

    let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));

    let func_iter = std::iter::from_fn(move || {
        if x <= y {
            let (old_x, old_y) = (x, y);
            x += 1;

            if d > 0 {
                y -= 1;
                d += 4 * (x - y) + 10;
            } else {
                d += 4 * x + 6;
            }
            Some(circle_sym(
                start,
                old_x,
                old_y,
                x_offset as i32,
                y_offset as i32,
                r,
            ))
        } else {
            None
        }
    });
    Box::new(func_iter)
}

fn circle_sym(
    start: egui::Pos2,
    x: i32,
    y: i32,
    x_off: i32,
    y_off: i32,
    r: i32,
) -> Vec<(Pixel, Pixel)> {
    let offsets = [
        (x, y),
        (-x, y),
        (x, -y),
        (-x, -y),
        (y, x),
        (-y, x),
        (y, -x),
        (-y, -x),
    ];

    let (start_x, start_y) = (start.x as i32, start.y as i32);
    offsets
        .into_iter()
        .map(|(dx, dy)| {
            (
                Pixel::new_i32(start_x + dx, start_y + dy, 255),
                Pixel::new_i32(
                    start_x + dx - x_off as i32 + r,
                    start_y + dy - y_off as i32 + r,
                    255,
                ),
            )
        })
        .collect()
}

pub fn paint_ellips(
    center: egui::Pos2,
    a: f32,
    b: f32,
) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
    let mut x = 0.0;
    let mut y = b;

    let mut d1 = (b * b) - (a * a * b) + (0.25 * a * a);
    let mut region1 = true;
    let (x_offset, y_offset) = (center.x, center.y);
    let mut d2 = 0.0;
    let func_iter = std::iter::from_fn(move || {
        if region1 {
            let (old_x, old_y) = (x, y);

            if d1 < 0.0 {
                d1 += b * b * (2.0 * x + 3.0);
            } else {
                d1 += b * b * (2.0 * x + 3.0) + a * a * (-2.0 * y + 2.0);
                y -= 1.0;
            }
            x += 1.0;
            region1 = b * b * x < a * a * y;
            if !region1 {
                d2 = b * b * (x + 0.5).powi(2) + a * a * (y - 1.0).powi(2) - a * a * b * b;
            }
            Some(quadrant_sym(center, old_x, old_y, a, b, x_offset, y_offset))
        } else if y >= 0.0 {
            let (old_x, old_y) = (x, y);
            if d2 > 0.0 {
                d2 += a * a * (-2.0 * y + 3.0);
            } else {
                d2 += a * a * (-2.0 * y + 3.0) + b * b * (2.0 * x + 2.0);
                x += 1.0;
            }
            y -= 1.0;
            return Some(quadrant_sym(center, old_x, old_y, a, b, x_offset, y_offset));
        } else {
            None
        }
    });
    Box::new(func_iter)
}

pub fn paint_hyperbola(
    center: egui::Pos2,
    a: f32,
    b: f32,
    max_iterations: u32,
) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
    let a_sq = (a * a) as i32;
    let b_sq = (b * b) as i32;
    let mut x = a as i32;
    let mut y = 0i32;
    let mut iteration = 0;
    let x_off = center.x - max_iterations as f32 - a * 2.0;
    let y_off = center.y - max_iterations as f32;

    let mut d1 = b_sq * (x * x - (x - 1) * (x - 1)) - a_sq * (y * y);

    let mut region1 = true;

    std::iter::from_fn(move || {
        if iteration >= max_iterations {
            return None;
        }
        iteration += 1;

        let (old_x, old_y) = (x, y);

        if region1 {
            if d1 < 0 {
                d1 += 2 * b_sq * (x + 1);
            } else {
                d1 += 2 * b_sq * (x + 1) - 2 * a_sq * (y + 1);
                y += 1;
            }
            x += 1;

            if b_sq * x > a_sq * y {
                region1 = false;
            }
        } else {
            let d2 = b_sq * (x + 1) * (x + 1) - a_sq * (y + 1) * (y + 1) - a_sq * b_sq;

            if d2 < 0 {
                x += 1;
            }
            y += 1;
        }

        Some(quadrant_sym(
            center,
            old_x as f32,
            old_y as f32,
            0.0,
            0.0,
            x_off,
            y_off,
        ))
    })
}

pub fn paint_parabola(
    center: egui::Pos2,
    p: f32,
    max_iterations: u32,
) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
    let (cx, cy) = (center.x, center.y);
    let p_int = p as i32;
    let mut x = 0i32;
    let mut y = 0i32;
    let mut iteration = 0;

    let x_off = cx - max_iterations as f32;
    let y_off = cy - max_iterations as f32;

    let mut d1 = 1 - 2 * p_int;

    let mut region1 = true;

    std::iter::from_fn(move || {
        if iteration >= max_iterations {
            return None;
        }
        iteration += 1;

        let (old_x, old_y) = (x, y);

        if region1 {
            if d1 < 0 {
                d1 += 2 * x + 3;
            } else {
                d1 += 2 * (x - p_int) + 3;
                y += 1;
            }
            x += 1;

            if x >= 2 * p_int {
                region1 = false;
            }
        } else {
            let d2 = (y + 1) * (y + 1) - 4 * p_int * (x + 1);
            if d2 < 0 {
                x += 1;
            }
            y += 1;
        }

        Some(quadrant_sym(
            center,
            old_x as f32,
            old_y as f32,
            0.0,
            0.0,
            x_off,
            y_off,
        ))
    })
}

fn quadrant_sym(
    center: egui::Pos2,
    x: f32,
    y: f32,
    a: f32,
    b: f32,
    x_off: f32,
    y_off: f32,
) -> Vec<(Pixel, Pixel)> {
    let offsets = [(x, y), (-x, y), (x, -y), (-x, -y)];

    offsets
        .into_iter()
        .map(|(dx, dy)| {
            (
                Pixel::new(center.x + dx, center.y + dy, 255),
                Pixel::new(
                    (center.x + dx - x_off + a).floor(),
                    (center.y + dy - y_off + b).floor(),
                    255,
                ),
            )
        })
        .collect()
}
