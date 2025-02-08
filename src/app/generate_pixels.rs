use super::figure_parameters;
use super::figure_parameters::LineType;
use super::parameters::ParameterState;
use crate::lines;
use crate::pixel::Pixel;
use crate::second_order_lines;
use eframe::egui::Pos2;

pub trait GeneratePixels {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2);
}

impl GeneratePixels for ParameterState {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2) {
        match self {
            ParameterState::Line(params) => params.generate_pixels(),
            ParameterState::Circle(params) => params.generate_pixels(),
            ParameterState::Ellips(params) => params.generate_pixels(),
            ParameterState::Hyperbola(params) => params.generate_pixels(),
            ParameterState::Parabola(params) => params.generate_pixels(),
        }
    }
}

impl GeneratePixels for figure_parameters::Line {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2) {
        if let figure_parameters::Line {
            start: Some(start),
            end: Some(end),
            line_type,
        } = self
        {
            let func =
                match line_type {
                    LineType::First => Box::new(lines::dda_line(*start, *end))
                        as Box<dyn Iterator<Item = Vec<Pixel>>>,
                    LineType::Second => Box::new(lines::bresenham_line(*start, *end))
                        as Box<dyn Iterator<Item = Vec<Pixel>>>,
                    LineType::Third => Box::new(lines::wu_line(*start, *end))
                        as Box<dyn Iterator<Item = Vec<Pixel>>>,
                };
            let offset = Pos2::new(start.x.min(end.x), start.y.min(end.y));
            return (func, offset);
        } else {
            (
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Vec<Pixel>>>,
                Pos2::default(),
            )
        }
    }
}

impl GeneratePixels for figure_parameters::Circle {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2) {
        if let figure_parameters::Circle {
            start: Some(start),
            end: Some(end),
        } = self
        {
            let r = start.distance(*end);
            let offset = Pos2::new(start.x - r, start.y - r);
            return (
                Box::new(second_order_lines::paint_circle(*start, *end)),
                offset,
            );
        }
        (Box::new(std::iter::empty()), Pos2::default())
    }
}

impl GeneratePixels for figure_parameters::Ellips {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2) {
        if let figure_parameters::Ellips {
            start: Some(center),
            a,
            b,
        } = self
        {
            let mut offset = center.clone();
            offset.x -= *a;
            offset.y -= *b;
            return (
                Box::new(second_order_lines::paint_ellips(*center, *a, *b)),
                offset,
            );
        }
        (Box::new(std::iter::empty()), Pos2::default())
    }
}

impl GeneratePixels for figure_parameters::Parabola {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2) {
        if let figure_parameters::Parabola {
            start: Some(center),
            p,
            max_iterations,
        } = self
        {
            let mut offset = center.clone();
            offset.x -= *max_iterations as f32;
            offset.y -= *max_iterations as f32;
            return (
                Box::new(second_order_lines::paint_parabola(
                    *center,
                    *p,
                    *max_iterations,
                )),
                offset,
            );
        }
        (Box::new(std::iter::empty()), Pos2::default())
    }
}

impl GeneratePixels for figure_parameters::Hyperbola {
    fn generate_pixels(&mut self) -> (Box<dyn Iterator<Item = Vec<Pixel>>>, Pos2) {
        if let figure_parameters::Hyperbola {
            start: Some(center),
            a,
            b,
            max_iterations,
        } = self
        {
            let mut offset = center.clone();
            offset.x -= *max_iterations as f32 + *a * 2.0;
            offset.y -= *max_iterations as f32;
            return (
                Box::new(second_order_lines::paint_hyperbola(
                    *center,
                    *a,
                    *b,
                    *max_iterations,
                )),
                offset,
            );
        }
        (Box::new(std::iter::empty()), Pos2::default())
    }
}
