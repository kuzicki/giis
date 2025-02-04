use eframe::egui;
#[derive(Clone)]
pub enum LineType {
    First,
    Second,
    Third,
}


pub struct Line {
    pub start: Option<egui::Pos2>,
    pub end: Option<egui::Pos2>,
    pub line_type: LineType,
}

impl Line {
    pub fn new(line_type: LineType) -> Self {
        Self {
            start: None,
            end: None,
            line_type,
        }
    }
}

#[derive(Default)]
pub struct Circle {
    pub start: Option<egui::Pos2>,
    pub end: Option<egui::Pos2>,
}

pub struct Ellips {
    pub start: Option<egui::Pos2>,
    pub a: f32,
    pub b: f32,
}

impl Default for Ellips {
    fn default() -> Self {
        Self {
            start: None,
            a: 15.0,
            b: 10.0,
        }
    }
}

pub struct Parabola {
    pub start: Option<egui::Pos2>,
    pub p: f32,
    pub max_iterations: u32
}

impl Default for Parabola {
    fn default() -> Self {
        Self {
            start: None,
            p: 15.0,
            max_iterations: 100
        }
    }
}

pub struct Hyperbola {
    pub start: Option<egui::Pos2>,
    pub a: f32,
    pub b: f32,
    pub max_iterations: u32
}

impl Default for Hyperbola {
    fn default() -> Self {
        Self {
            start: None,
            a: 15.0,
            b: 10.0,
            max_iterations: 100,
        }
    }
}
