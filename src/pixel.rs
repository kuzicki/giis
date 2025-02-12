use eframe::egui;

#[derive(Debug, Clone)]
pub struct Pixel {
    pub pos: egui::Pos2,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub intensity: u8,
}

impl Pixel {
    pub fn new_black(x: f32, y: f32, intensity: u8) -> Self {
        Self {
            pos: egui::Pos2::new(x, y),
            red: 0,
            green: 0,
            blue: 0,
            intensity,
        }
    }

    pub fn new(x: f32, y: f32, rgba: (u8, u8, u8, u8)) -> Self {
        Self {
            pos: egui::Pos2::new(x, y),
            red: rgba.0,
            green: rgba.1,
            blue: rgba.2,
            intensity: rgba.3,
        }
    }

    pub fn new_pos2(pos: egui::Pos2, rgba: (u8, u8, u8, u8)) -> Self {
        Self {
            pos,
            red: rgba.0,
            green: rgba.1,
            blue: rgba.2,
            intensity: rgba.3,
        }
    }

    pub fn new_black_i32(x: i32, y: i32, intensity: u8) -> Self {
        Self {
            pos: egui::Pos2::new(x as f32, y as f32),
            red: 0,
            green: 0,
            blue: 0,
            intensity,
        }
    }

    pub fn from_pos_black(pos: egui::Pos2, intensity: u8) -> Self {
        Self {
            pos,
            red: 0,
            green: 0,
            blue: 0,
            intensity,
        }
    }
}
