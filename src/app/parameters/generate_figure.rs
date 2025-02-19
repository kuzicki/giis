use super::super::figure::{Circle, Curve, Ellips, Hyperbola, Line, Object, Parabola};
use super::figure_parameters;
use super::figure_parameters::LineType;
use super::Figure;
use super::ParameterState;
use crate::lines;
use crate::pixel::Pixel;
use crate::second_order_lines;
use eframe::egui::Pos2;

pub trait GenerateFigure {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>>;
}

impl GenerateFigure for ParameterState {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
        use ParameterState as ps;
        match self {
            ps::Line(params) => params.generate_figure(),
            ps::Circle(params) => params.generate_figure(),
            ps::Ellips(params) => params.generate_figure(),
            ps::Hyperbola(params) => params.generate_figure(),
            ps::Parabola(params) => params.generate_figure(),
            ps::Curve(params) => params.generate_figure(),
            ps::Object(params) => params.generate_figure(),
        }
    }
}

impl GenerateFigure for figure_parameters::Line {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
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
            Some(Box::new(Line::new(func, offset)))
        } else {
            None
        }
    }
}

impl GenerateFigure for figure_parameters::Circle {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
        if let figure_parameters::Circle {
            start: Some(start),
            end: Some(end),
        } = self
        {
            let r = start.distance(*end);
            let offset = Pos2::new(start.x - r, start.y - r);
            let func = Box::new(second_order_lines::paint_circle(*start, *end));
            Some(Box::new(Circle::new(func, offset)))
        } else {
            None
        }
    }
}

impl GenerateFigure for figure_parameters::Ellips {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
        if let figure_parameters::Ellips {
            start: Some(center),
            a,
            b,
        } = self
        {
            let mut offset = center.clone();
            offset.x -= *a;
            offset.y -= *b;
            let func = Box::new(second_order_lines::paint_ellips(*center, *a, *b));
            Some(Box::new(Ellips::new(func, offset)))
        } else {
            None
        }
    }
}

impl GenerateFigure for figure_parameters::Parabola {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
        if let figure_parameters::Parabola {
            start: Some(center),
            p,
            max_iterations,
        } = self
        {
            let mut offset = center.clone();
            offset.x -= *max_iterations as f32;
            offset.y -= *max_iterations as f32;

            let func = Box::new(second_order_lines::paint_parabola(
                *center,
                *p,
                *max_iterations,
            ));
            Some(Box::new(Parabola::new(func, offset)))
        } else {
            None
        }
    }
}

impl GenerateFigure for figure_parameters::Hyperbola {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
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
            let func = Box::new(second_order_lines::paint_hyperbola(
                *center,
                *a,
                *b,
                *max_iterations,
            ));
            Some(Box::new(Hyperbola::new(func, offset)))
        } else {
            None
        }
    }
}

impl GenerateFigure for figure_parameters::Curve {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
        let figure_parameters::Curve {
            control_points: ref points,
            curve_type,
        } = self;

        let &[first, second, third, fourth] = points.as_slice() else {
            return None;
        };

        Some(Box::new(Curve::new(
            [first, second, third, fourth],
            curve_type.clone(),
        )))
    }
}

impl GenerateFigure for figure_parameters::Object {
    fn generate_figure(&mut self) -> Option<Box<dyn Figure>> {
        if let figure_parameters::Object {
            start: Some(pos),
            file_path,
        } = self
        {
            return Some(Box::new(Object::new(file_path, *pos)));
        }
        None
    }
}
