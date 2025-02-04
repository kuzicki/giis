use crate::pixel::Pixel;
use eframe::egui;
pub(super) mod figure_parameters;

pub enum ParameterState {
    Line(figure_parameters::Line),
    Circle(figure_parameters::Circle),
    Ellips(figure_parameters::Ellips),
    Hyperbola(figure_parameters::Hyperbola),
    Parabola(figure_parameters::Parabola),
}

impl Default for ParameterState {
    fn default() -> Self {
        Self::Line(figure_parameters::Line::new(
            figure_parameters::LineType::First,
        ))
    }
}

impl From<&Action> for ParameterState {
    fn from(figure: &Action) -> Self {
        use figure_parameters as fp;
        use Action::*;
        use ParameterState as ps;
        match figure {
            DrawDDA | DrawBresenham | DrawVu => match figure {
                DrawDDA => ps::Line(fp::Line::new(fp::LineType::First)),
                DrawBresenham => ps::Line(fp::Line::new(fp::LineType::Second)),
                DrawVu => ps::Line(fp::Line::new(fp::LineType::Third)),
                _ => unreachable!(),
            },
            DrawCircle => ps::Circle(fp::Circle::default()),
            DrawEllips => ps::Ellips(fp::Ellips::default()),
            DrawHyperbola => ps::Hyperbola(fp::Hyperbola::default()),
            DrawParabola => ps::Parabola(fp::Parabola::default()),
        }
    }
}

pub struct DrawingState {
    pub points: Vec<Pixel>,
    pub processing_func: Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>,
    pub mode: Mode,
    pub selected: Action,
    pub parameters: ParameterState,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            selected: Action::DrawDDA,
            processing_func: Box::new(std::iter::empty()),
            mode: Mode::Awaiting,
            parameters: ParameterState::default(),
        }
    }
}

pub struct DebugState {
    pub enabled: bool,
    pub points: Vec<Pixel>,
    pub start: Option<egui::Pos2>,
    pub end: Option<egui::Pos2>,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            enabled: false,
            points: Vec::new(),
            start: None,
            end: None,
        }
    }
}

pub struct ExecutionControl {
    pub update_interval: std::time::Duration,
    pub last_update: Option<std::time::Instant>,
    pub paused: bool,
}

impl Default for ExecutionControl {
    fn default() -> Self {
        Self {
            update_interval: std::time::Duration::from_millis(10),
            last_update: None,
            paused: false,
        }
    }
}

pub struct ViewportSettings {
    pub offset: egui::Vec2,
    pub debug_scale: f32,
    pub scroll_offset: egui::Vec2,
}

impl Default for ViewportSettings {
    fn default() -> Self {
        Self {
            offset: egui::Vec2::default(),
            debug_scale: 10.0,
            scroll_offset: egui::Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Action {
    DrawDDA,
    DrawBresenham,
    DrawVu,
    DrawCircle,
    DrawEllips,
    DrawHyperbola,
    DrawParabola,
}

impl Action {
    pub fn to_str(&self) -> &str {
        use Action as fg;
        match self {
            fg::DrawDDA => "DDA Line",
            fg::DrawBresenham => "Bresenham's Line",
            fg::DrawVu => "Vu Line",
            fg::DrawCircle => "Circle",
            fg::DrawEllips => "Ellips",
            fg::DrawHyperbola => "Hyperbola",
            fg::DrawParabola => "Parabola",
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Awaiting,
    Computing,
}
