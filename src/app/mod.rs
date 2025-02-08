use eframe::egui;
mod click_action;
mod debug_window;
mod generate_pixels;
mod parameter_dialog;
mod parameters;
use click_action::ClickAction;
use generate_pixels::GeneratePixels;
use parameter_dialog::FigureParameters;
use parameters::*;

pub struct PaintApp {
    drawing: DrawingState,
    debug: DebugState,
    execution: ExecutionControl,
    viewport: ViewportSettings,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            drawing: DrawingState::default(),
            debug: DebugState::default(),
            execution: ExecutionControl::default(),
            viewport: ViewportSettings::default(),
        }
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.checkbox(&mut self.debug.enabled, "Debug mode");
            self.figure_combobox(ui);

            if ui.button("Clear").clicked() && !matches!(self.drawing.mode, Mode::Computing) {
                self.debug.points.clear();
                self.drawing.points.clear();
            }
            self.update_computation(ctx);
            self.main_painter(ui);
            self.show_parameter_dialog(ctx);
        });
        self.debug_window(ctx);
    }
}

impl PaintApp {
    fn main_painter(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            let response = ui.allocate_rect(rect, egui::Sense::click());

            let painter = ui.painter_at(rect);
            if response.clicked() {
                if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                    self.handle_painter_click(pos);
                }
            }

            painter.rect_filled(rect, 0.0, egui::Color32::LIGHT_BLUE);
            for pixel in &mut self.drawing.points {
                let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, pixel.intensity);
                let rect = egui::Rect::from_min_size(pixel.pos, egui::Vec2::new(1.0, 1.0));
                painter.rect_filled(rect, 0.0, color);
            }
        });
    }

    fn figure_combobox(&mut self, ui: &mut egui::Ui) {
        let previous = self.drawing.selected.clone();
        egui::ComboBox::from_label("")
            .selected_text(format!("{}", self.drawing.selected.to_str()))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.drawing.selected, Action::DrawDDA, "DDA Line");
                ui.selectable_value(
                    &mut self.drawing.selected,
                    Action::DrawBresenham,
                    "Bresenham's Line",
                );
                ui.selectable_value(&mut self.drawing.selected, Action::DrawVu, "Vu Line");
                ui.selectable_value(&mut self.drawing.selected, Action::DrawCircle, "Circle");
                ui.selectable_value(&mut self.drawing.selected, Action::DrawEllips, "Ellips");
                ui.selectable_value(
                    &mut self.drawing.selected,
                    Action::DrawHyperbola,
                    "Hyperbola",
                );
                ui.selectable_value(&mut self.drawing.selected, Action::DrawParabola, "Parabola");
            });
        if previous != self.drawing.selected && !matches!(self.drawing.mode, Mode::Computing) {
            self.drawing.parameters = ParameterState::from(&self.drawing.selected)
        }
    }

    fn update_computation(&mut self, ctx: &egui::Context) {
        if matches!(self.drawing.mode, Mode::Computing) {
            if self.debug.enabled {
                if self.execution.paused {
                    return;
                }
                let now = std::time::Instant::now();
                if self.execution.last_update.map_or(true, |last| {
                    now.duration_since(last) >= self.execution.update_interval
                }) {
                    self.execution.last_update = Some(now);
                } else {
                    ctx.request_repaint();
                    return;
                }
                if let Some(pixels) = self.drawing.processing_func.next() {
                    for pixel in pixels {
                        let debug_pixel = pixel.clone();
                        self.debug.points.push(debug_pixel);
                        self.drawing.points.push(pixel);
                    }
                } else {
                    self.drawing.mode = Mode::Awaiting;
                }
            } else {
                for pixels in self.drawing.processing_func.as_mut() {
                    for pixel in pixels {
                        let debug_pixel = pixel.clone();
                        self.debug.points.push(debug_pixel);
                        self.drawing.points.push(pixel);
                    }
                }
                self.drawing.mode = Mode::Awaiting;
            }
            ctx.request_repaint();
        }
    }

    fn handle_painter_click(&mut self, pos: egui::Pos2) {
        if !matches!(self.drawing.mode, Mode::Awaiting) {
            return;
        }

        if self.drawing.parameters.handle_click(pos) {
            self.start_computing();
        }
    }

    fn start_computing(&mut self) {
        self.debug.points.clear();
        self.drawing.mode = Mode::Computing;
        (self.drawing.processing_func, self.viewport.offset) =
            self.drawing.parameters.generate_pixels();
        self.drawing.parameters = ParameterState::from(&self.drawing.selected)
    }

    fn show_parameter_dialog(&mut self, ctx: &egui::Context) {
        if self.drawing.parameters.show_dialog(ctx) {
            self.start_computing();
        }
    }
}
