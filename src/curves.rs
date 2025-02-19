use crate::pixel::Pixel;
use eframe::egui::{Pos2, Vec2};

const SCALE: f32 = 1.8;
const MIN_SCALE: f32 = 20.0;

fn matrix_multiply(a: &[[f32; 4]; 4], b: &[f32; 4]) -> [f32; 4] {
    let mut result = [0.0; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i] += a[i][j] * b[j];
        }
    }
    result
}

fn multiply_coeffs(coeffs: &[f32; 4], t_vec: &[f32; 4]) -> f32 {
    let mut result = 0.0;
    for i in 0..4 {
        result += coeffs[i] * t_vec[i];
    }
    result
}

pub fn generate_hermite_curve(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, pixels: &mut Vec<Pixel>) {
    let t0 = Vec2::new((p1.x - p0.x) * 3.0, (p1.y - p0.y) * 3.0);
    let t1 = Vec2::new((p3.x - p2.x) * 3.0, (p3.y - p2.y) * 3.0);

    let points = [p0, p1, p2, p3];
    let max_distance = points
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| points.iter().skip(i + 1).map(move |&p2| p1.distance(p2)))
        .fold(0.0, f32::max);
    let steps = (max_distance * SCALE).max(MIN_SCALE) as usize;
    let hermite_matrix = [
        [2.0, -2.0, 1.0, 1.0],
        [-3.0, 3.0, -2.0, -1.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
    ];

    let x_coeffs = matrix_multiply(&hermite_matrix, &[p0.x, p2.x, t0.x, t1.x]);
    let y_coeffs = matrix_multiply(&hermite_matrix, &[p0.y, p2.y, t0.y, t1.y]);

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let t_vec = [t * t * t, t * t, t, 1.0];

        let x = multiply_coeffs(&x_coeffs, &t_vec);
        let y = multiply_coeffs(&y_coeffs, &t_vec);

        pixels.push(Pixel::new_black(x, y, 255));
    }
}

pub fn generate_bezier_curve(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, pixels: &mut Vec<Pixel>) {
    let points = [p0, p1, p2, p3];
    let max_distance = points
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| points.iter().skip(i + 1).map(move |&p2| p1.distance(p2)))
        .fold(0.0, f32::max);
    let steps = (max_distance * SCALE).max(MIN_SCALE) as usize;
    let bezier_matrix: [[f32; 4]; 4] = [
        [-1.0, 3.0, -3.0, 1.0],
        [3.0, -6.0, 3.0, 0.0],
        [-3.0, 3.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
    ];
    let control_points_x = [p0.x, p1.x, p2.x, p3.x];
    let control_points_y = [p0.y, p1.y, p2.y, p3.y];

    for i in 0..=steps {
        let t = i as f32 / steps as f32;

        let t_vector = [t.powi(3), t.powi(2), t, 1.0];

        let x_coeffs = matrix_multiply(&bezier_matrix, &control_points_x);
        let y_coeffs = matrix_multiply(&bezier_matrix, &control_points_y);

        let x = multiply_coeffs(&x_coeffs, &t_vector);
        let y = multiply_coeffs(&y_coeffs, &t_vector);

        pixels.push(Pixel::new_black(x, y, 255));
    }
}

fn matrix_multiply_4x4_2x4(matrix: &[[f32; 4]; 4], points: &[[f32; 2]; 4]) -> [[f32; 2]; 4] {
    let mut result = [[0.0; 2]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][0] += matrix[i][j] * points[j][0];
            result[i][1] += matrix[i][j] * points[j][1];
        }
    }
    result
}

pub fn generate_bspline_curve(control_points: &[Pos2], pixels: &mut Vec<Pixel>) {
    let n = control_points.len();
    if n < 4 {
        panic!("At least 4 control points required.");
    }

    let max_distance = control_points
        .iter()
        .enumerate()
        .flat_map(|(i, &p1)| {
            control_points
                .iter()
                .skip(i + 1)
                .map(move |&p2| p1.distance(p2))
        })
        .fold(0.0, f32::max);

    let steps = (max_distance * SCALE).max(MIN_SCALE) as usize;
    let mut augmented_points = Vec::with_capacity(n + 3);
    augmented_points.extend_from_slice(control_points);
    augmented_points.push(control_points[0]);
    augmented_points.push(control_points[1]);
    augmented_points.push(control_points[2]);

    let m_bspline = [
        [-1.0 / 6.0, 3.0 / 6.0, -3.0 / 6.0, 1.0 / 6.0],
        [3.0 / 6.0, -6.0 / 6.0, 3.0 / 6.0, 0.0],
        [-3.0 / 6.0, 0.0, 3.0 / 6.0, 0.0],
        [1.0 / 6.0, 4.0 / 6.0, 1.0 / 6.0, 0.0],
    ];

    for i in 0..n {
        let p = [
            [augmented_points[i].x, augmented_points[i].y],
            [augmented_points[i + 1].x, augmented_points[i + 1].y],
            [augmented_points[i + 2].x, augmented_points[i + 2].y],
            [augmented_points[i + 3].x, augmented_points[i + 3].y],
        ];

        let g = matrix_multiply_4x4_2x4(&m_bspline, &p);

        for j in 0..=steps {
            let t = j as f32 / steps as f32;
            let t_vec = [t.powi(3), t.powi(2), t, 1.0];

            let mut x = 0.0;
            let mut y = 0.0;
            for k in 0..4 {
                x += t_vec[k] * g[k][0];
                y += t_vec[k] * g[k][1];
            }

            pixels.push(Pixel::new_black(x, y, 255));
        }
    }
}
