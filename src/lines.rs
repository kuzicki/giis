use crate::pixel::Pixel;
use eframe::egui;

pub fn dda_line(start: egui::Pos2, end: egui::Pos2) -> impl Iterator<Item = Vec<Pixel>> {
    let length = (end.x - start.x).abs().max((end.y - start.y).abs());
    let dx = (end.x - start.x) / length;
    let dy = (end.y - start.y) / length;

    let mut x = start.x + 0.5 * sign(dx);
    let mut y = start.y + 0.5 * sign(dy);

    let mut i = 0.0;
    let first_value = std::iter::once(vec![(Pixel::new_black(x, y, 255))]);
    let func_iter = std::iter::from_fn(move || {
        if i <= length {
            let current = Pixel::new_black(x.floor(), y.floor(), 255);
            x = x + dx;
            y = y + dy;
            i += 1.0;
            Some(vec![current])
        } else {
            None
        }
    });

    first_value.chain(func_iter)
}

pub fn bresenham_line(start: egui::Pos2, end: egui::Pos2) -> impl Iterator<Item = Vec<Pixel>> {
    let mut x = start.x.round() as i32;
    let mut y = start.y.round() as i32;
    let end_x = end.x.round() as i32;
    let end_y = end.y.round() as i32;

    let dx = (end_x - x).abs();
    let dy = (end_y - y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };

    let mut err = dx - dy;
    let first_value = std::iter::once(vec![(Pixel::new_black(x as f32, y as f32, 255))]);
    let func_iter = std::iter::from_fn(move || {
        if x == end_x && y == end_y {
            return None;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }

        let (x, y) = (x as f32, y as f32);
        return Some(vec![(Pixel::new_black(x, y, 255))]);
    });
    first_value.chain(func_iter)
}

fn get_intensity(
    x_current: f32,
    y_intercept: f32,
    normalized_intensity: f32,
    steep: bool,
) -> Vec<Pixel> {
    let intensity = (255.0 * normalized_intensity) as u8;
    let inverted_intensity = (255.0 * (1.0 - normalized_intensity)) as u8;
    if steep {
        vec![
            Pixel::new_black(y_intercept.floor(), x_current.floor(), inverted_intensity),
            Pixel::new_black(y_intercept.floor() + 1.0, x_current.floor(), intensity),
        ]
    } else {
        vec![
            Pixel::new_black(x_current.floor(), y_intercept.floor(), inverted_intensity),
            Pixel::new_black(x_current.floor(), y_intercept.floor() + 1.0, intensity),
        ]
    }
}

pub fn wu_line(start: egui::Pos2, end: egui::Pos2) -> impl Iterator<Item = Vec<Pixel>> {
    let x = start.x;
    let y = start.y;
    let end_x = end.x;
    let end_y = end.y;

    let dx = end_x - x;
    let dy = end_y - y;

    let steep = dy.abs() > dx.abs();

    let (x, y, end_x, end_y) = if steep {
        (y, x, end_y, end_x)
    } else {
        (x, y, end_x, end_y)
    };

    let (x, end_x, y, end_y) = if x > end_x {
        (end_x, x, end_y, y)
    } else {
        (x, end_x, y, end_y)
    };

    let dx = end_x - x;
    let dy = end_y - y;
    let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

    let mut y_intercept = y + gradient;

    let mut first = true;
    let mut x_current = x + 1.0;
    let func_iter = std::iter::from_fn(move || {
        if first {
            first = false;
            if steep {
                return Some(vec![(Pixel::new_black(y.floor(), x.floor(), 255))]);
            } else {
                return Some(vec![(Pixel::new_black(x.floor(), y.floor(), 255))]);
            }
        }

        if x_current <= end_x {
            let intensity = y_intercept - y_intercept.floor();
            let prev_current_x = x_current;
            let prev_intercept_y = y_intercept;

            y_intercept += gradient;
            x_current += 1.0;

            return Some(get_intensity(
                prev_current_x,
                prev_intercept_y,
                intensity,
                steep,
            ));
        } else {
            None
        }
    });
    return func_iter;
}

fn sign(number: f32) -> f32 {
    if number > 0.0 {
        1.0
    } else if number < 0.0 {
        -1.0
    } else {
        0.0
    }
}
