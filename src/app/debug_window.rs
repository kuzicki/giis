use super::figure::Debuggable;
use super::Mode;
use eframe::egui;
use eframe::egui::{Painter, Rect};

impl super::PaintApp {
    pub(super) fn debug_window(&mut self, ctx: &egui::Context) {
        if matches!(self.drawing.mode, Mode::Debug) {
            egui::Window::new("Debug window").show(ctx, |ui| {
                let mut scale = self.viewport.debug_scale as i32;
                ui.add(
                    egui::Slider::new(&mut scale, 2..=50)
                        .text("Scale")
                        .trailing_fill(true),
                );
                self.viewport.debug_scale = scale as f32;
                let mut speed = self.execution.update_interval.as_millis() as u64;
                ui.add(
                    egui::Slider::new(&mut speed, 1..=1000)
                        .text("Update interval(ms)")
                        .trailing_fill(true),
                );
                self.execution.update_interval = std::time::Duration::from_millis(speed);
                ui.horizontal(|ui| {
                    if ui
                        .button(if self.execution.paused {
                            "Resume"
                        } else {
                            "Pause"
                        })
                        .clicked()
                    {
                        self.execution.paused = !self.execution.paused;
                    }
                    ui.add(
                        egui::Slider::new(&mut self.viewport.scroll_offset.x, 0.0..=500.0)
                            .show_value(false)
                            .text("Horizontal scroll"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.viewport.scroll_offset.y, 0.0..=500.0)
                            .show_value(false)
                            .text("Vertical scroll"),
                    );
                });
                self.draw_debug_grid(ui);
            });
        }
    }

    fn draw_debug_grid(&mut self, ui: &mut egui::Ui) {
        let grid_size: f32 = self.viewport.debug_scale as f32;

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        let rect = response.rect;
        let window_center = rect.left_top();

        if let Some(index) = self.debug.figure_index {
            if let Some(figure) = self.drawing.figures[index].as_debug() {
                self.draw_figure_debug(figure, grid_size, window_center, &painter);
            }
        }

        self.draw_grid(&rect, &painter, grid_size);
    }

    fn draw_figure_debug(
        &self,
        figure: &dyn Debuggable,
        grid_size: f32,
        window_center: egui::Pos2,
        painter: &Painter,
    ) {
        let offset = figure.get_offset();
        let pixels = figure.get_pixels();
        for pixel in pixels {
            let grid_point = egui::Pos2::new(
                (((pixel.pos.x - offset.x) + 2.0 - self.viewport.scroll_offset.x).floor()
                    * grid_size)
                    + window_center.x,
                (((pixel.pos.y - offset.y) + 2.0 - self.viewport.scroll_offset.y).floor()
                    * grid_size)
                    + window_center.y,
            );
            let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, pixel.intensity);
            let rect =
                egui::Rect::from_min_size(grid_point, egui::Vec2::new(1.0, 1.0) * grid_size * 0.95);
            painter.rect_filled(rect, 0.0, color);
        }
    }

    fn draw_grid(&self, rect: &Rect, painter: &Painter, grid_size: f32) {
        let grid_color = egui::Color32::from_gray(200);
        for x in (rect.left() as i32..rect.right() as i32).step_by(grid_size as usize) {
            painter.line_segment(
                [
                    egui::Pos2::new(x as f32, rect.top()),
                    egui::Pos2::new(x as f32, rect.bottom()),
                ],
                egui::Stroke::new(1.0, grid_color),
            );
        }

        for y in (rect.top() as i32..rect.bottom() as i32).step_by(grid_size as usize) {
            painter.line_segment(
                [
                    egui::Pos2::new(rect.left(), y as f32),
                    egui::Pos2::new(rect.right(), y as f32),
                ],
                egui::Stroke::new(1.0, grid_color),
            );
        }
    }
}
