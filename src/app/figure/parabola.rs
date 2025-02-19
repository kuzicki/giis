use super::{Debuggable, Figure, Drawable, draw_pixels};
use crate::pixel::Pixel;
use eframe::egui::Pos2;

pub struct Parabola {
    points: Vec<Pixel>,
    update_func: Box<dyn Iterator<Item = Vec<Pixel>>>,
    debug_offset: Pos2,
}

impl Parabola {
    pub fn new(update_func: Box<dyn Iterator<Item = Vec<Pixel>>>, offset: Pos2) -> Self {
        Self {
            points: vec![],
            update_func,
            debug_offset: offset,
        }
    }
}

impl Figure for Parabola {
    fn as_debug_mut(&mut self) -> Option<&mut dyn Debuggable> {
        Some(self)
    }

    fn as_debug(&self) -> Option<&dyn Debuggable> {
        Some(self)
    }
}

impl_debuggable!(Parabola, update_func, points, debug_offset);


impl Drawable for Parabola {
    fn draw(&self, painter: &eframe::egui::Painter) {
        draw_pixels(&self.points, painter);
    }
}
