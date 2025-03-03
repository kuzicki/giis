use super::figure_parameters;
use super::ParameterState;
use eframe::egui;

pub trait KeyboardAction {
    fn handle_key(&mut self, i: &egui::InputState) -> bool;
}

impl KeyboardAction for ParameterState {
    fn handle_key(&mut self, i: &egui::InputState) -> bool {
        use ParameterState as ps;
        match self {
            ps::Polygon(params) => params.handle_key(i),
            ps::Voronoi(params) => params.handle_key(i),
            ps::Delone(params) => params.handle_key(i),
            _ => false,
        }
    }
}

impl KeyboardAction for figure_parameters::Polygon {
    fn handle_key(&mut self, i: &egui::InputState) -> bool {
        if i.key_pressed(egui::Key::Enter) && self.points.len() > 1 {
            return true;
        }
        false
    }
}

impl KeyboardAction for figure_parameters::Delone {
    fn handle_key(&mut self, i: &egui::InputState) -> bool {
        if i.key_pressed(egui::Key::Enter) && self.points.len() > 1 {
            return true;
        }
        false
    }
}

impl KeyboardAction for figure_parameters::Voronoi {
    fn handle_key(&mut self, i: &egui::InputState) -> bool {
        if i.key_pressed(egui::Key::Enter) && self.points.len() > 1 {
            return true;
        }
        false
    }
}
