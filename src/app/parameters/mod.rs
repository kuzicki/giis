use super::figure::CurveType;
use super::figure::{ConnectableCurves, EditableControlPoints, Figure, Selectable};
use eframe::egui;
pub(super) mod click_action;
pub(super) mod figure_parameters;
pub(super) mod generate_figure;
pub(super) mod mode_panel;

pub enum ParameterState {
    Line(figure_parameters::Line),
    Circle(figure_parameters::Circle),
    Ellips(figure_parameters::Ellips),
    Hyperbola(figure_parameters::Hyperbola),
    Parabola(figure_parameters::Parabola),
    Curve(figure_parameters::Curve),
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
            DrawHermite | DrawBezier | DrawBSpline => match figure {
                DrawHermite => ps::Curve(fp::Curve::new(CurveType::Hermite)),
                DrawBezier => ps::Curve(fp::Curve::new(CurveType::Bezier)),
                DrawBSpline => ps::Curve(fp::Curve::new(CurveType::BSpline)),
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Clone)]
pub enum Mode {
    None,
    Debug,
    ConnectCurve(Option<usize>),
    MoveControlPoints(Option<usize>),
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mode::None, Mode::None) => true,
            (Mode::Debug, Mode::Debug) => true,
            (Mode::ConnectCurve(_), Mode::ConnectCurve(_)) => true,
            (Mode::MoveControlPoints(_), Mode::MoveControlPoints(_)) => true,
            _ => false,
        }
    }
}

impl Mode {
    pub fn change_to(&mut self, figures: &mut Vec<Box<dyn Figure>>, new_mode: Mode) {
        match self {
            Mode::ConnectCurve(Some(index)) | Mode::MoveControlPoints(Some(index)) => {
                if let Some(selectable) = figures[*index].as_selectable_mut() {
                    selectable.deselect();
                }
            }
            _ => (),
        }
        *self = new_mode;
    }

    pub fn reset(&mut self) {
        *self = match self {
            Mode::ConnectCurve(Some(_)) => Mode::ConnectCurve(None),
            Mode::MoveControlPoints(Some(_)) => Mode::MoveControlPoints(None),
            _ => self.clone(),
        }
    }
}

pub struct DrawingState {
    pub figures: Vec<Box<dyn Figure>>,
    pub selected_figure: Option<usize>,
    pub status: Status,
    pub selected: Action,
    pub parameters: ParameterState,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            selected: Action::DrawDDA,
            status: Status::Awaiting,
            parameters: ParameterState::default(),
            figures: vec![],
            selected_figure: None,
        }
    }
}

pub struct DebugState {
    pub figure_index: Option<usize>,
}

impl Default for DebugState {
    fn default() -> Self {
        Self { figure_index: None }
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
    pub debug_scale: f32,
    pub scroll_offset: egui::Vec2,
}

impl Default for ViewportSettings {
    fn default() -> Self {
        Self {
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
    DrawHermite,
    DrawBezier,
    DrawBSpline,
}

impl Action {
    pub fn variants() -> &'static [Action] {
        &[
            Action::DrawDDA,
            Action::DrawBresenham,
            Action::DrawVu,
            Action::DrawCircle,
            Action::DrawEllips,
            Action::DrawHyperbola,
            Action::DrawParabola,
            Action::DrawHermite,
            Action::DrawBezier,
            Action::DrawBSpline,
        ]
    }

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
            fg::DrawHermite => "Hermite curve",
            fg::DrawBezier => "Bezier curve",
            fg::DrawBSpline => "B-spline curve",
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Awaiting,
    Computing,
}
