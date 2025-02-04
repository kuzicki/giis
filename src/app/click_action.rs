use super::figure_parameters;
use super::parameters::ParameterState;
use eframe::egui;

pub trait ClickAction {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool;
}

impl ClickAction for ParameterState {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        match self {
            ParameterState::Line(params) => params.handle_click(pos),
            ParameterState::Circle(params) => params.handle_click(pos),
            ParameterState::Ellips(params) => params.handle_click(pos),
            ParameterState::Hyperbola(params) => params.handle_click(pos),
            ParameterState::Parabola(params) => params.handle_click(pos),
        }
    }
}

impl ClickAction for figure_parameters::Line {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        if self.start.is_none() {
            self.start = Some(pos);
        } else if self.end.is_none() {
            self.end = Some(pos);
            return true;
        }
        false
    }
}

impl ClickAction for figure_parameters::Circle {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        if self.start.is_none() {
            self.start = Some(pos);
        } else if self.end.is_none() {
            self.end = Some(pos);
            return true;
        }
        false
    }
}

impl ClickAction for figure_parameters::Ellips {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        if self.start.is_none() {
            self.start = Some(pos);
        }
        false
    }
}

impl ClickAction for figure_parameters::Parabola {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        if self.start.is_none() {
            self.start = Some(pos);
        }
        false
    }
}

impl ClickAction for figure_parameters::Hyperbola {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        if self.start.is_none() {
            self.start = Some(pos);
        }
        false
    }
}
