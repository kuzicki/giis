mod pixel;
use eframe::egui;
use pixel::Pixel;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GIIS",
        options,
        Box::new(|_cc| Ok(Box::new(PaintApp::default()))),
    )
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum LineMode {
    First,
    Second,
    Third,
}

#[derive(Debug)]
enum Mode {
    Awaiting,
    Computing,
}

struct PaintApp {
    start: Option<egui::Pos2>,
    end: Option<egui::Pos2>,
    points: Vec<Pixel>,
    selected: LineMode,
    proccessing_func: Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>>,
    mode: Mode,
    debug_mode: bool,
    debug_points: Vec<Pixel>,
    update_interval: std::time::Duration,
    last_update: Option<std::time::Instant>,
    pause: bool,
    x_offset: f32,
    y_offset: f32,
    debug_scale: f32,
    scroll_offset: egui::Vec2,
    debug_start: Option<egui::Pos2>,
    debug_end: Option<egui::Pos2>
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            points: Vec::new(),
            selected: LineMode::First,
            proccessing_func: Box::new(std::iter::empty()),
            mode: Mode::Awaiting,
            debug_mode: false,
            debug_points: Vec::new(),
            update_interval: std::time::Duration::from_millis(10),
            last_update: None,
            pause: false,
            x_offset: 0.0,
            y_offset: 0.0,
            debug_scale: 10.0,
            scroll_offset: egui::Vec2::new(0.0, 0.0),
            debug_start: None,
            debug_end: None,
        }
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Click two points to draw a line with custom brightness per dot.");
            ui.checkbox(&mut self.debug_mode, "Debug mode");
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", self.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, LineMode::First, "First");
                    ui.selectable_value(&mut self.selected, LineMode::Second, "Second");
                    ui.selectable_value(&mut self.selected, LineMode::Third, "Third");
                });

            self.update_computation(ctx);
            egui::Frame::group(ui.style()).show(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                let response = ui.allocate_rect(rect, egui::Sense::click());

                let painter = ui.painter_at(rect);
                if response.clicked() {
                    println!("Click event!");
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        self.handle_painter_click(pos);
                    }
                }

                painter.rect_filled(rect, 0.0, egui::Color32::LIGHT_BLUE);
                for pixel in &mut self.points {
                    let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, pixel.intensity);
                    let rect = egui::Rect::from_min_size(pixel.pos, egui::Vec2::new(1.0, 1.0));
                    painter.rect_filled(rect, 0.0, color);
                }
            });
        });
        self.debug_window(ctx);
    }
}

fn sign(number: f32) -> f32 {
    if number > 0.0 {
        1.0
    } else if number < 0.0 {
        -1.0
    } else {
        0.0
    }
}

impl PaintApp {
    fn update_computation(&mut self, ctx: &egui::Context) {
        if matches!(self.mode, Mode::Computing) {
            if self.debug_mode {
                if self.pause {
                    return;
                }
                let now = std::time::Instant::now();
                if self.last_update.map_or(true, |last| {
                    now.duration_since(last) >= self.update_interval
                }) {
                    self.last_update = Some(now);
                } else {
                    ctx.request_repaint();
                    return;
                }
                if let Some(pixels) = self.proccessing_func.next() {
                    for (pixel, debug_pixel) in pixels {
                        self.points.push(pixel);
                        self.debug_points.push(debug_pixel);
                    }
                } else {
                    self.mode = Mode::Awaiting;
                }
            } else {
                for pixels in self.proccessing_func.as_mut() {
                    println!("Pixels: {:?}", pixels);
                    for (pixel, debug_pixel) in pixels {
                        self.points.push(pixel);
                        self.debug_points.push(debug_pixel);
                    }
                    println!("Debug pixels: {:?}", self.debug_points);
                    println!("pixels: {:?}", self.points);
                }
                self.mode = Mode::Awaiting;
            }
            ctx.request_repaint();
        }
    }

    fn debug_window(&mut self, ctx: &egui::Context) {
        if self.debug_mode {
            egui::Window::new("Debug window").show(ctx, |ui| {
                let mut scale = self.debug_scale as i32;
                ui.add(
                    egui::Slider::new(&mut scale, 5..=50)
                        .text("Scale")
                        .trailing_fill(true),
                );
                self.debug_scale = scale as f32;
                let mut speed = self.update_interval.as_millis() as u64;
                ui.add(
                    egui::Slider::new(&mut speed, 1..=1000)
                        .text("Update interval(ms)")
                        .trailing_fill(true),
                );
                self.update_interval = std::time::Duration::from_millis(speed);
                ui.horizontal(|ui| {
                    if ui
                        .button(if self.pause { "Resume" } else { "Pause" })
                        .clicked()
                    {
                        self.pause = !self.pause;
                    }
                    ui.add(
                        egui::Slider::new(&mut self.scroll_offset.x, 0.0..=500.0)
                            .show_value(false)
                            .text("Horizontal scroll"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.scroll_offset.y, 0.0..=500.0)
                            .show_value(false)
                            .text("Vertical scroll"),
                    );
                });
                self.draw_debug_grid(ui);
            });
        }
    }

    fn handle_painter_click(&mut self, pos: egui::Pos2) {
        if !matches!(self.mode, Mode::Awaiting) {
            return;
        }

        if self.start.is_none() {
            self.start = Some(pos);
        } else if self.end.is_none() {
            self.end = Some(pos);
            if let (Some(start), Some(end)) = (self.start, self.end) {
                self.proccessing_func = Box::new(self.generate_points(start, end));
                self.mode = Mode::Computing;
                self.start = None;
                self.end = None;
            }
        }
    }

    fn generate_points(
        &mut self,
        start: egui::Pos2,
        end: egui::Pos2,
    ) -> Box<dyn Iterator<Item = Vec<(Pixel, Pixel)>>> {
        match self.selected {
            LineMode::First => Box::new(self.first(start, end)),
            LineMode::Second => Box::new(self.second(start, end)),
            LineMode::Third => Box::new(self.third(start, end)),
        }
    }

    fn first(
        &mut self,
        start: egui::Pos2,
        end: egui::Pos2,
    ) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
        let length = (end.x - start.x).abs().max((end.y - start.y).abs());
        let dx = (end.x - start.x) / length;
        let dy = (end.y - start.y) / length;

        let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));
        let (x_max_offset, y_max_offset) = (start.x.max(end.x), start.y.max(end.y));
        self.x_offset = x_offset;
        self.y_offset = y_offset;

        let mut x = start.x + 0.5 * sign(dx);
        let mut y = start.y + 0.5 * sign(dy);
        self.debug_points.clear();

        // self.points.push(Pixel::new(x, y, 255));
        // self.debug_points.push(Pixel::new(
        //     (x - x_offset).floor(),
        //     (y - y_offset).floor(),
        //     255,
        // ));
        let mut i = 0.0;
        let mut first = true;
        let func_iter = std::iter::from_fn(move || {
            if first {
                first = false;
                return Some(vec![(
                    Pixel::new(x, y, 255),
                    Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255),
                )]);
            }
            if i <= length {
                let current = Pixel::new(x.floor(), y.floor(), 255);
                let debug = Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255);
                x = x + dx;
                y = y + dy;
                i += 1.0;
                Some(vec![(current, debug)])
            } else {
                None
            }
        });

        return func_iter;
    }

    fn second(
        &mut self,
        start: egui::Pos2,
        end: egui::Pos2,
    ) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
        self.debug_points.clear();
        let mut x = start.x.round() as i32;
        let mut y = start.y.round() as i32;
        let end_x = end.x.round() as i32;
        let end_y = end.y.round() as i32;
        let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));
        self.x_offset = x_offset;
        self.y_offset = y_offset;

        let dx = (end_x - x).abs();
        let dy = (end_y - y).abs();
        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };

        let mut err = dx - dy;
        let mut first = true;

        let func_iter = std::iter::from_fn(move || {
            if first {
                first = false;
                let (x, y) = (x as f32, y as f32);
                return Some(vec![(
                    Pixel::new(x, y, 255),
                    Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255),
                )]);
            }
            if x == end_x && y == end_y {
                return None;
            }

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }

            let (x, y) = (x as f32, y as f32);
            return Some(vec![(
                Pixel::new(x, y, 255),
                Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255),
            )]);
        });
        return func_iter;
    }

    fn get_intensity(
        x_current: f32,
        y_intercept: f32,
        x_offset: f32,
        y_offset: f32,
        normalized_intensity: f32,
        steep: bool,
    ) -> Vec<(Pixel, Pixel)> {
        let intensity = (255.0 * normalized_intensity) as u8;
        let inverted_intensity = (255.0 * (1.0 - normalized_intensity)) as u8;
        if steep {
            vec![
                (
                    Pixel::new(y_intercept.floor(), x_current.floor(), inverted_intensity),
                    Pixel::new(
                        (y_intercept - x_offset).floor(),
                        (x_current - y_offset).floor(),
                        inverted_intensity,
                    ),
                ),
                (
                    Pixel::new(y_intercept.floor() + 1.0, x_current.floor(), intensity),
                    Pixel::new((y_intercept + 1.0 - x_offset).floor(), (x_current - y_offset).floor(), intensity),
                ),
            ]
        } else {
            vec![
                (
                    Pixel::new(x_current.floor(), y_intercept.floor(), inverted_intensity),
                    Pixel::new(
                        (x_current - x_offset).floor(),
                        (y_intercept - y_offset).floor(),
                        inverted_intensity,
                    ),
                ),
                (
                    Pixel::new(x_current.floor(), y_intercept.floor() + 1.0, intensity),
                    Pixel::new((x_current - x_offset).floor(), (y_intercept + 1.0 - y_offset).floor(), intensity),
                ),
            ]
        }
    }

    fn third(
        &mut self,
        start: egui::Pos2,
        end: egui::Pos2,
    ) -> impl Iterator<Item = Vec<(Pixel, Pixel)>> {
        let mut x = start.x;
        let mut y = start.y;
        let end_x = end.x;
        let end_y = end.y;
        let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));
        self.x_offset = x_offset;
        self.y_offset = y_offset;
        self.debug_points.clear();

        let dx = end_x - x;
        let dy = end_y - y;

        // Determine the primary axis (X or Y)
        let steep = dy.abs() > dx.abs();

        // Swap x and y if the line is steep (slope > 1)
        let (x, y, end_x, end_y) = if steep {
            (y, x, end_y, end_x)
        } else {
            (x, y, end_x, end_y)
        };

        // Ensure the line is drawn from left to right
        let (x, end_x, y, end_y) = if x > end_x {
            (end_x, x, end_y, y)
        } else {
            (x, end_x, y, end_y)
        };

        let dx = end_x - x;
        let dy = end_y - y;
        let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

        let mut y_intercept = y + gradient;

        let mut first = true;
        let mut x_current = x + 1.0;
        let func_iter = std::iter::from_fn(move || {
            if first {
                first = false;
                if steep {
                    println!("steep: {x}, {y}, {x_offset}, {y_offset}");
                    return Some(vec![(
                        Pixel::new(y.floor(), x.floor(), 255),
                        Pixel::new((y - x_offset).floor(), (x - y_offset).floor(), 255),
                    )]);
                } else {
                    println!("not steep: {x}, {y}, {x_offset}, {y_offset}");
                    return Some(vec![(
                        Pixel::new(x.floor(), y.floor(), 255),
                        Pixel::new((x - x_offset).floor(), (y - y_offset).floor(), 255),
                    )]);
                }
            }

            // Iterate along the primary axis (x)
            if x_current <= end_x {
                let intensity = y_intercept - y_intercept.floor();
                println!("Intensity: {}", intensity);
                let prev_current_x = x_current;
                let prev_intercept_y = y_intercept;
                // if steep {
                // self.plot_with_intensity(
                //     y_intercept.floor() as i32,
                //     x_current as i32,
                //     1.0 - intensity,
                // );
                // self.plot_with_intensity(
                //     y_intercept.floor() as i32 + 1,
                //     x_current as i32,
                //     intensity,
                // );
                // } else {
                //     self.plot_with_intensity(
                //         x_current as i32,
                //         y_intercept.floor() as i32,
                //         1.0 - intensity,
                //     );
                //     self.plot_with_intensity(
                //         x_current as i32,
                //         y_intercept.floor() as i32 + 1,
                //         intensity,
                //     );
                // }

                y_intercept += gradient;
                x_current += 1.0;

                return Some(Self::get_intensity(
                    prev_current_x,
                    prev_intercept_y,
                    x_offset,
                    y_offset,
                    intensity,
                    steep,
                ));
            } else {
                None
            }
        });
        return func_iter;
    }

    fn plot_with_intensity(&mut self, pixel: Pixel, debug_pixel: Pixel) {
        self.points.push(pixel);
        self.debug_points.push(debug_pixel);
    }

    fn draw_debug_grid(&mut self, ui: &mut egui::Ui) {
        let grid_size: f32 = self.debug_scale as f32;
        let grid_color = egui::Color32::from_gray(200);

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

        let rect = response.rect;

        let mut window_center = rect.left_top();

        for (pixel) in &self.debug_points {
            let grid_point = egui::Pos2::new(
                ((pixel.pos.x + 2.0 - self.scroll_offset.x) * grid_size) + window_center.x,
                ((pixel.pos.y + 2.0 - self.scroll_offset.y) * grid_size) + window_center.y,
            );
            let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, pixel.intensity);
            let rect =
                egui::Rect::from_min_size(grid_point, egui::Vec2::new(1.0, 1.0) * grid_size * 0.95);
            painter.rect_filled(rect, 0.0, color);
        }

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
