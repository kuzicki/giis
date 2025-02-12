use crate::pixel::Pixel;
use eframe::egui::Pos2;
#[macro_use]
mod macros;
mod circle;
mod curve;
mod ellips;
mod hyperbola;
mod line;
mod parabola;
pub use circle::Circle;
pub use curve::{CurveType, Curve};
pub use ellips::Ellips;
pub use hyperbola::Hyperbola;
pub use line::Line;
pub use parabola::Parabola;

pub trait Figure {
    fn get_pixels(&self) -> &[Pixel];

    fn as_selectable(&self) -> Option<&dyn Selectable> {
        None
    }

    fn as_selectable_mut(&mut self) -> Option<&mut dyn Selectable> {
        None
    }

    fn as_debug_mut(&mut self) -> Option<&mut dyn Debuggable> {
        None
    }

    fn as_debug(&self) -> Option<&dyn Debuggable> {
        None
    }

    fn as_editable_points(&self) -> Option<&dyn EditableControlPoints> {
        None
    }

    fn as_editable_points_mut(&mut self) -> Option<&mut dyn EditableControlPoints> {
        None
    }
}

pub trait Selectable: Figure {
    fn select(&mut self);
    fn deselect(&mut self);

    fn bounding_box(&self) -> (Pos2, Pos2);

    fn hit_test(&self, pos: Pos2) -> bool;
}

pub trait EditableControlPoints: Selectable {
    fn control_points(&self) -> &[Pos2];
    fn control_points_mut(&mut self) -> &mut [Pos2];
    fn hit_test_control_point(&self, pos: Pos2, radius: f32) -> Option<usize>;
    fn move_point(&mut self, pos: Pos2) -> bool;
    fn toggle_point(&mut self, index: usize);
}

pub trait ConnectableCurves: EditableControlPoints {
    fn connect_curves(&mut self, other: &mut Self);
}

pub trait Debuggable: Figure {
    fn update_frame(&mut self) -> bool;
    fn evaluate(&mut self);
    fn get_offset(&self) -> Pos2;
}
