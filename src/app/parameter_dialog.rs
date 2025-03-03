use super::figure_parameters;
use super::parameters::ParameterState;
use eframe::egui;
use rfd::FileDialog;

pub trait FigureParameters {
    fn show_dialog(&mut self, _ctx: &egui::Context) -> bool {
        false
    }
}

impl FigureParameters for ParameterState {
    fn show_dialog(&mut self, ctx: &egui::Context) -> bool {
        use ParameterState as ps;
        match self {
            ps::Line(..) => false,
            ps::Circle(..) => false,
            ps::Ellips(ellips) => ellips.show_dialog(ctx),
            ps::Hyperbola(hyperbola) => hyperbola.show_dialog(ctx),
            ps::Parabola(parabola) => parabola.show_dialog(ctx),
            ps::Curve(..) => false,
            ps::Object(object) => object.show_dialog(ctx),
            ps::Polygon(..) => false,
            ps::Delone(..) => false,
            ps::Voronoi(..) => false
        }
    }
}

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

impl FigureParameters for figure_parameters::Object {
    fn show_dialog(&mut self, ctx: &egui::Context) -> bool {
        let mut apply_changes = false;
        if self.start.is_some() {
            egui::Window::new("Open file(.obj)").show(ctx, |ui| {
                let selected_file = if self.file_path != "" {
                    format!("Selected file: {}", self.file_path)
                } else {
                    format!("No file selected")
                };
                if ui.button("Select file").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("OBJ files", &["obj"])
                        .pick_file()
                    {
                        self.file_path = path.to_string_lossy().to_string();
                    }
                }
                ui.label(selected_file);
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
