use super::figure_parameters;
use super::parameters::ParameterState;
use eframe::egui;

pub trait FigureParameters {
    fn show_dialog(&mut self, _ctx: &egui::Context) -> bool {
        false
    }
}

impl FigureParameters for ParameterState {
    fn show_dialog(&mut self, ctx: &egui::Context) -> bool {
        use ParameterState as ps;
        match self {
            ps::Line(line) => line.show_dialog(ctx),
            ps::Circle(circle) => circle.show_dialog(ctx),
            ps::Ellips(ellips) => ellips.show_dialog(ctx),
            ps::Hyperbola(hyperbola) => hyperbola.show_dialog(ctx),
            ps::Parabola(parabola) => parabola.show_dialog(ctx),
            ps::Curve(curve) => curve.show_dialog(ctx)
        }
    }
}

impl FigureParameters for figure_parameters::Curve {}

impl FigureParameters for figure_parameters::Line {}

impl FigureParameters for figure_parameters::Circle {}

impl FigureParameters for figure_parameters::Ellips {
    fn show_dialog(&mut self, ctx: &egui::Context) -> bool {
        let mut apply_changes = false;
        if self.start.is_some() {
            egui::Window::new("Ellipse Parameters").show(ctx, |ui| {
                ui.label("Set Ellipse Parameters:");
                ui.add(egui::DragValue::new(&mut self.a).speed(1.0).prefix("A: "));
                ui.add(egui::DragValue::new(&mut self.b).speed(1.0).prefix("B: "));

                if ui.button("Apply").clicked() {
                    apply_changes = true;
                }

                if ui.button("Cancel").clicked() {
                    self.start = None;
                }
            });
        }
        apply_changes
    }
}

impl FigureParameters for figure_parameters::Hyperbola {
    fn show_dialog(&mut self, ctx: &egui::Context) -> bool {
        let mut apply_changes = false;
        if self.start.is_some() {
            egui::Window::new("Hyperbola Parameters").show(ctx, |ui| {
                ui.label("Set Hyperbola Parameters:");
                ui.add(
                    egui::DragValue::new(&mut self.a)
                        .speed(1.0)
                        .prefix("A: ")
                        .range(1..=10000),
                );
                ui.add(
                    egui::DragValue::new(&mut self.b)
                        .speed(1.0)
                        .prefix("B: ")
                        .range(1..=10000),
                );
                ui.add(
                    egui::DragValue::new(&mut self.max_iterations)
                        .speed(1.0)
                        .prefix("Iterations")
                        .range(1..=1000),
                );

                if ui.button("Apply").clicked() {
                    apply_changes = true;
                }

                if ui.button("Cancel").clicked() {
                    self.start = None;
                }
            });
        }
        apply_changes
    }
}

impl FigureParameters for figure_parameters::Parabola {
    fn show_dialog(&mut self, ctx: &egui::Context) -> bool {
        let mut apply_changes = false;
        if self.start.is_some() {
            egui::Window::new("Parabola Parameters").show(ctx, |ui| {
                ui.label("Set Parabola Parameters:");
                ui.add(
                    egui::DragValue::new(&mut self.p)
                        .speed(1.0)
                        .prefix("P: ")
                        .range(1..=10000),
                );
                ui.add(
                    egui::DragValue::new(&mut self.max_iterations)
                        .speed(1.0)
                        .prefix("Iterations")
                        .range(1..=1000),
                );

                if ui.button("Apply").clicked() {
                    apply_changes = true;
                }

                if ui.button("Cancel").clicked() {
                    self.start = None;
                }
            });
        }
        apply_changes
    }
}
