use super::figure_parameters;
use super::figure_parameters::LineType;
use super::parameters::ParameterState;
use crate::lines;
use crate::pixel::Pixel;
use crate::second_order_lines;

pub trait GeneratePixels {
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>;
}

impl GeneratePixels for ParameterState {
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
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
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
        if let figure_parameters::Line {
            start: Some(start),
            end: Some(end),
            line_type,
        } = self
        {
            match line_type {
                LineType::First => Box::new(lines::dda_line(*start, *end))
                    as Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>,
                LineType::Second => Box::new(lines::bresenham_line(*start, *end))
                    as Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>,
                LineType::Third => Box::new(lines::wu_line(*start, *end))
                    as Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>,
            }
        } else {
            Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>
        }
    }
}

impl GeneratePixels for figure_parameters::Circle {
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
        if let figure_parameters::Circle {
            start: Some(start),
            end: Some(end),
        } = self
        {
            return Box::new(second_order_lines::paint_circle(*start, *end));
        }
        Box::new(std::iter::empty())
    }
}

impl GeneratePixels for figure_parameters::Ellips {
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
        if let figure_parameters::Ellips {
            start: Some(center),
            a,
            b,
        } = self
        {
            return Box::new(second_order_lines::paint_ellips(*center, *a, *b));
        }
        Box::new(std::iter::empty())
    }
}

impl GeneratePixels for figure_parameters::Parabola {
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
        if let figure_parameters::Parabola {
            start: Some(center),
            p,
            max_iterations,
        } = self
        {
            // todo!();
            return Box::new(second_order_lines::paint_parabola(
                *center,
                *p,
                *max_iterations,
            ));
        }
        Box::new(std::iter::empty())
    }
}

impl GeneratePixels for figure_parameters::Hyperbola {
    fn generate_pixels(&mut self) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
        if let figure_parameters::Hyperbola {
            start: Some(center),
            a,
            b,
            max_iterations,
        } = self
        {
            // todo!();
            return Box::new(second_order_lines::paint_hyperbola(
                *center,
                *a,
                *b,
                *max_iterations,
            ));
        }
        Box::new(std::iter::empty())
    }
}
