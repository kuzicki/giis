use super::{Debuggable, Figure};
use crate::pixel::Pixel;
use eframe::egui::Pos2;

pub struct Hyperbola {
    points: Vec<Pixel>,
    update_func: Box<dyn Iterator<Item = Vec<Pixel>>>,
    debug_offset: Pos2,
}

impl Hyperbola {
    pub fn new(update_func: Box<dyn Iterator<Item = Vec<Pixel>>>, offset: Pos2) -> Self {
        Self {
            points: vec![],
            update_func,
            debug_offset: offset,
        }
    }
}

impl Figure for Hyperbola {
    fn get_pixels(&self) -> &[Pixel] {
        self.points.as_slice()
    }

    fn as_debug_mut(&mut self) -> Option<&mut dyn Debuggable> {
        Some(self)
    }

    fn as_debug(&self) -> Option<&dyn Debuggable> {
        Some(self)
    }
}

impl_debuggable!(Hyperbola, update_func, points, debug_offset);
