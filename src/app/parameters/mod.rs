use super::figure::CurveType;
use super::figure::{Figure, Selectable};
use eframe::egui;
pub(super) mod click_action;
pub(super) mod figure_parameters;
pub(super) mod generate_figure;
pub(super) mod keyboard_action;

pub enum ParameterState {
    Line(figure_parameters::Line),
    Circle(figure_parameters::Circle),
    Ellips(figure_parameters::Ellips),
    Hyperbola(figure_parameters::Hyperbola),
    Parabola(figure_parameters::Parabola),
    Curve(figure_parameters::Curve),
    Object(figure_parameters::Object),
    Polygon(figure_parameters::Polygon),
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
            LoadObject => ps::Object(fp::Object::new()),
            DrawPolygon => ps::Polygon(fp::Polygon::new()),
        }
    }
}

#[derive(Clone)]
pub enum PolygonTest {
    Line(Option<egui::Pos2>),
    Dot,
    None,
}

#[derive(Clone)]
pub enum Mode {
    None,
    Debug,
    MoveControlPoints(Option<usize>),
    TransformObject(Option<usize>),
    PolygonOperations(Option<usize>, PolygonTest),
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Mode::None, Mode::None) => true,
            (Mode::Debug, Mode::Debug) => true,
            (Mode::MoveControlPoints(_), Mode::MoveControlPoints(_)) => true,
            (Mode::PolygonOperations(..), Mode::PolygonOperations(..)) => true,
            _ => false,
        }
    }
}

impl Mode {
    fn change_to(&mut self, figures: &mut Vec<Box<dyn Figure>>, new_mode: Mode) {
        match self {
            Mode::MoveControlPoints(Some(index))
            | Mode::TransformObject(Some(index))
            | Mode::PolygonOperations(Some(index), _) => {
                if let Some(selectable) = figures[*index].as_selectable_mut() {
                    selectable.deselect();
                }
            }
            _ => (),
        }
        *self = new_mode;
    }

    fn reset(&mut self) {
        *self = match self {
            Mode::MoveControlPoints(Some(_)) => Mode::MoveControlPoints(None),
            Mode::TransformObject(Some(_)) => Mode::TransformObject(None),
            Mode::PolygonOperations(Some(_), _) => Mode::PolygonOperations(None, PolygonTest::None),
            _ => self.clone(),
        }
    }
}

pub struct DrawingState {
    pub mode: Mode,
    pub figures: Vec<Box<dyn Figure>>,
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
            mode: Mode::None,
        }
    }
}

impl DrawingState {
    pub fn change_mode(&mut self, new_mode: Mode) {
        self.mode.change_to(&mut self.figures, new_mode);
    }

    pub fn reset(&mut self) {
        self.mode.reset();
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
    pub modal_window_text: String
}

impl Default for ViewportSettings {
    fn default() -> Self {
        Self {
            debug_scale: 10.0,
            scroll_offset: egui::Vec2::new(0.0, 0.0),
            modal_window_text: String::new()
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
    LoadObject,
    DrawPolygon,
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
            Action::LoadObject,
            Action::DrawPolygon,
        ]
    }

    pub fn to_str(&self) -> &str {
        use Action as act;
        match self {
            act::DrawDDA => "DDA Line",
            act::DrawBresenham => "Bresenham's Line",
            act::DrawVu => "Vu Line",
            act::DrawCircle => "Circle",
            act::DrawEllips => "Ellips",
            act::DrawHyperbola => "Hyperbola",
            act::DrawParabola => "Parabola",
            act::DrawHermite => "Hermite curve",
            act::DrawBezier => "Bezier curve",
            act::DrawBSpline => "B-spline curve",
            act::LoadObject => "3D object transforms",
            act::DrawPolygon => "Polygons",
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Awaiting,
    Computing,
}
