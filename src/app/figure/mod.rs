use crate::pixel::Pixel;
use eframe::egui::{InputState, Color32, Painter, Pos2, Rect, Vec2};
#[macro_use]
mod macros;
mod circle;
mod curve;
mod ellips;
mod hyperbola;
mod line;
mod object;
mod parabola;
pub use circle::Circle;
pub use curve::{Curve, CurveType};
pub use ellips::Ellips;
pub use hyperbola::Hyperbola;
pub use line::Line;
pub use object::Object;
pub use parabola::Parabola;

pub trait Figure: Drawable {

    fn as_selectable(&self) -> Option<&dyn Selectable> {
        None
    }

    fn as_selectable_mut(&mut self) -> Option<&mut dyn Selectable> {
        None
    }

    fn as_transformable(&self) -> Option<&dyn Transformable> {
        None
    }

    fn as_transformable_mut(&mut self) -> Option<&mut dyn Transformable> {
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
    fn get_pixels(&self) -> &[Pixel];
}

pub trait Transformable: Selectable {
    fn handle_keyboard(&mut self, ctx: &InputState);
}

pub trait Drawable {
    fn draw(&self, painter: &Painter);
}

pub(self) fn draw_pixels(pixels: &Vec<Pixel>, painter: &Painter) {
    for pixel in pixels {
        let color =
            Color32::from_rgba_premultiplied(pixel.red, pixel.green, pixel.blue, pixel.intensity);
        let rect = Rect::from_min_size(pixel.pos, Vec2::new(1.0, 1.0));
        painter.rect_filled(rect, 0.0, color);
    }
}
