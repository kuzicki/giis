use eframe::egui;

#[derive(Debug)]
pub struct Pixel {
    pub pos: egui::Pos2,
    pub intensity: u8,
}

impl Pixel {
    pub fn new(x: f32, y: f32, intensity: u8) -> Self {
        Self {
            pos: egui::Pos2::new(x, y),
            intensity,
        }
    }

    pub fn from_pos(pos: egui::Pos2, intensity: u8) -> Self {
        Self { pos, intensity }
    }
}
