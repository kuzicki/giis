use super::{draw_pixels, Drawable, Figure};
use crate::pixel::Pixel;
use eframe::egui::Pos2;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub struct Voronoi {}

impl Voronoi {
    pub fn new(points: Vec<Pos2>) -> Self {
        Self {}
    }
}

impl Figure for Voronoi {}

impl Drawable for Voronoi {
    fn draw(&self, painter: &eframe::egui::Painter) {}
}
