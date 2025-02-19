use eframe::egui;
mod debug_window;
mod figure;
mod parameter_dialog;
mod parameters;
use click_action::ClickAction;
use generate_figure::GenerateFigure;
use mode_panel::ModePanel;
use parameter_dialog::FigureParameters;
use parameters::*;

pub struct PaintApp {
    drawing: DrawingState,
    debug: DebugState,
    mode: Mode,
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
            mode: Mode::None,
        }
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.handle_keyboard(ctx);
            self.show_panel(ui);
            self.figure_combobox(ui);

            if ui.button("Clear").clicked() && !matches!(self.drawing.status, Status::Computing) {
                self.mode.reset();
                self.drawing.figures.clear();
                self.debug.figure_index = None;
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
            for figure in self.drawing.figures.iter() {
                figure.draw(&painter);
            }
        });
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::T) {
                match (&self.drawing.parameters, &mut self.mode) {
                    (ParameterState::Object(_), Mode::None | Mode::TransformObject(Some(_))) => {
                        self.mode
                            .change_to(&mut self.drawing.figures, Mode::TransformObject(None));
                    }
                    (ParameterState::Object(_), Mode::TransformObject(None)) => {
                        self.mode.change_to(&mut self.drawing.figures, Mode::None);
                    }
                    _ => (),
                }
            } else {
                if let Mode::TransformObject(Some(index)) = self.mode {
                    if let Some(figure) = self.drawing.figures[index].as_transformable_mut() {
                        figure.handle_keyboard(i);
                    }
                }
            }
        });
        ctx.request_repaint();
    }

    fn show_panel(&mut self, ui: &mut egui::Ui) {
        self.drawing
            .parameters
            .show_panel(ui, &mut self.mode, &mut self.drawing.figures);
    }

    fn figure_combobox(&mut self, ui: &mut egui::Ui) {
        let previous = self.drawing.selected.clone();
        egui::ComboBox::from_label("")
            .selected_text(format!("{}", self.drawing.selected.to_str()))
            .show_ui(ui, |ui| {
                for action in Action::variants() {
                    ui.selectable_value(
                        &mut self.drawing.selected,
                        action.clone(),
                        action.to_str(),
                    );
                }
            });
        if previous != self.drawing.selected && !matches!(self.drawing.status, Status::Computing) {
            self.mode.change_to(&mut self.drawing.figures, Mode::None);
            self.drawing.parameters = ParameterState::from(&self.drawing.selected)
        }
    }

    fn update_computation(&mut self, ctx: &egui::Context) {
        if matches!(self.drawing.status, Status::Computing) {
            if matches!(self.mode, Mode::Debug) {
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
                if let Some(index) = self.debug.figure_index {
                    if let Some(debug_figure) = self.drawing.figures[index].as_debug_mut() {
                        if !debug_figure.update_frame() {
                            self.drawing.status = Status::Awaiting;
                        }
                    }
                }
            }
            ctx.request_repaint();
        }
    }

    fn handle_painter_click(&mut self, pos: egui::Pos2) {
        if !matches!(self.drawing.status, Status::Awaiting) {
            return;
        }
        match self.mode {
            Mode::None | Mode::Debug => {
                if self.drawing.parameters.handle_click(pos) {
                    self.start_computing();
                }
            }
            Mode::MoveControlPoints(ref mut index) => {
                if let Some(index_inner) = index {
                    if let Some(editable) =
                        self.drawing.figures[*index_inner].as_editable_points_mut()
                    {
                        if let Some(point_index) = editable.hit_test_control_point(pos, 3.0) {
                            editable.toggle_point(point_index);
                        } else {
                            if !editable.move_point(pos) {
                                *index = None;
                            }
                        }
                    }
                } else {
                    for (new_index, figure) in self.drawing.figures.iter_mut().enumerate() {
                        if let Some(target) = figure.as_editable_points_mut() {
                            if target.hit_test(pos) {
                                target.select();
                                *index = Some(new_index);
                                break;
                            }
                        }
                    }
                }
            }
            Mode::ConnectCurve(_index) => {}
            Mode::TransformObject(ref mut index) => {
                if let None = index {
                    for (new_index, figure) in self.drawing.figures.iter_mut().enumerate() {
                        if let Some(target) = figure.as_transformable_mut() {
                            if target.hit_test(pos) {
                                target.select();
                                *index = Some(new_index);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn start_computing(&mut self) {
        let mut new_figure = self
            .drawing
            .parameters
            .generate_figure()
            .expect("Generating figure only on events");
        if matches!(self.mode, Mode::Debug) {
            self.drawing.status = Status::Computing;
        } else {
            if let Some(figure) = new_figure.as_debug_mut() {
                figure.evaluate();
            }
        }
        self.debug.figure_index = Some(self.drawing.figures.len());
        self.drawing.figures.push(new_figure);
        self.drawing.parameters = ParameterState::from(&self.drawing.selected)
    }

    fn show_parameter_dialog(&mut self, ctx: &egui::Context) {
        if self.drawing.parameters.show_dialog(ctx) {
            self.start_computing();
        }
    }
}
