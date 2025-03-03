use super::figure_parameters;
use super::ParameterState;
use eframe::egui;

pub trait ClickAction {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool;
}

impl ClickAction for ParameterState {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        use ParameterState as ps;
        match self {
            ps::Line(params) => params.handle_click(pos),
            ps::Circle(params) => params.handle_click(pos),
            ps::Ellips(params) => params.handle_click(pos),
            ps::Hyperbola(params) => params.handle_click(pos),
            ps::Parabola(params) => params.handle_click(pos),
            ps::Curve(params) => params.handle_click(pos),
            ps::Object(params) => params.handle_click(pos),
            ps::Polygon(params) => params.handle_click(pos),
            ps::Delone(params) => params.handle_click(pos),
            ps::Voronoi(params) => params.handle_click(pos),
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

impl ClickAction for figure_parameters::Curve {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        self.control_points.push(pos);
        self.control_points.len() == 4
    }
}

impl ClickAction for figure_parameters::Object {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        if self.start.is_none() {
            self.start = Some(pos);
        }
        false
    }
}

impl ClickAction for figure_parameters::Polygon {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        self.points.push(pos);
        false
    }
}

impl ClickAction for figure_parameters::Delone {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        self.points.push(pos);
        false
    }
}

impl ClickAction for figure_parameters::Voronoi {
    fn handle_click(&mut self, pos: egui::Pos2) -> bool {
        self.points.push(pos);
        false
    }
}
