use eframe::egui;

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

struct PaintApp {
    start: Option<egui::Pos2>,
    end: Option<egui::Pos2>,
    points: Vec<(egui::Pos2, u8)>,
    selected: LineMode,
    debug_mode: bool,
    debug_points: Vec<(egui::Pos2, u8)>,
    pause: bool,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            points: Vec::new(),
            selected: LineMode::First,
            debug_mode: false,
            debug_points: Vec::new(),
            pause: false
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

            egui::Frame::group(ui.style()).show(ui, |ui| {
                let rect = ui.available_rect_before_wrap();
                let response = ui.allocate_rect(rect, egui::Sense::click());

                let painter = ui.painter_at(rect);
                if response.clicked() {
                    println!("Click event!");
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        if self.start.is_none() {
                            self.start = Some(pos);
                        } else if self.end.is_none() {
                            self.end = Some(pos);
                            if let (Some(start), Some(end)) = (self.start, self.end) {
                                self.generate_points(start, end, self.selected.clone());
                                self.start = None;
                                self.end = None;
                            }
                        }
                    }
                }

                painter.rect_filled(rect, 0.0, egui::Color32::LIGHT_BLUE);
                for (point, brightness) in &mut self.points {
                    let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, *brightness);
                    let rect = egui::Rect::from_min_size(*point, egui::Vec2::new(1.0, 1.0));
                    painter.rect_filled(rect, 0.0, color);
                }
                if self.debug_mode {
                    egui::Window::new("Debug window").show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .button(if self.pause { "Resume" } else { "Pause" })
                                .clicked()
                            {
                                self.pause = !self.pause;
                            }
                        });
                        self.draw_debug_grid(ui);
                    });
                }
            })
        });
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
    fn generate_points(&mut self, start: egui::Pos2, end: egui::Pos2, line_mode: LineMode) {
        match line_mode {
            LineMode::First => self.first(start, end),
            LineMode::Second => self.second(start, end),
            LineMode::Third => self.third(start, end),
        }
    }

    fn first(&mut self, start: egui::Pos2, end: egui::Pos2) {
        let length = (end.x - start.x).abs().max((end.y - start.y).abs());
        let dx = (end.x - start.x) / length;
        let dy = (end.y - start.y) / length;

        let (x_offset, y_offset) = (start.x.min(end.x), start.y.min(end.y));
        let (x_max_offset, y_max_offset) = (start.x.max(end.x), start.y.max(end.y));

        let mut x = start.x + 0.5 * sign(dx);
        let mut y = start.y + 0.5 * sign(dy);

        self.points
            .push((egui::Pos2::new(x.floor(), y.floor()), 255));
        self.debug_points.clear();
        self.debug_points.push((
            egui::Pos2::new((x - x_offset).floor(), (y - y_offset)).floor(),
            255,
        ));
        let mut i = 0.0;
        while i <= length {
            if self.pause {
                continue;
            }
            x = x + dx;
            y = y + dy;
            self.points
                .push((egui::Pos2::new(x.floor(), y.floor()), 255));
            self.debug_points.push((
                egui::Pos2::new((x - x_offset).floor(), (y - y_offset).floor()),
                255,
            ));
            i += 1.0;
        }
        println!("i: {i}");
        println!("Len: {:#?}", self.points.len());
        println!("Debug len: {:#?}", self.debug_points.len());
    }

    fn second(&mut self, start: egui::Pos2, end: egui::Pos2) {
        // Convert to integers for Bresenham's algorithm
        let mut x = start.x.round() as i32;
        let mut y = start.y.round() as i32;
        let end_x = end.x.round() as i32;
        let end_y = end.y.round() as i32;

        let dx = (end_x - x).abs();
        let dy = (end_y - y).abs();
        let sx = if start.x < end.x { 1 } else { -1 };
        let sy = if start.y < end.y { 1 } else { -1 };

        let mut err = dx - dy;

        // Push the first point
        self.points.push((egui::Pos2::new(x as f32, y as f32), 255));

        loop {
            if x == end_x && y == end_y {
                break;
            }

            let e2 = 2 * err;

            // Adjust the `x` and `y` based on the error term
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }

            // Push the current point to the list
            self.points.push((egui::Pos2::new(x as f32, y as f32), 255));
        }
        println!("Points: {:?}", self.points);
    }

    fn third(&mut self, start: egui::Pos2, end: egui::Pos2) {
        // Convert to floating-point coordinates for Wu's algorithm
        let mut x = start.x;
        let mut y = start.y;
        let end_x = end.x;
        let end_y = end.y;

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

        // Initial y-intercept
        let mut y_intercept = y + gradient;

        // Draw the first point
        if steep {
            self.plot_with_intensity(y as i32, x as i32, 1.0);
            self.plot_with_intensity(y as i32 + 1, x as i32, 0.0);
        } else {
            self.plot_with_intensity(x as i32, y as i32, 1.0);
            self.plot_with_intensity(x as i32, y as i32 + 1, 0.0);
        }

        // Iterate along the primary axis (x)
        let mut x_current = x + 1.0;
        while x_current <= end_x {
            let intensity = y_intercept - y_intercept.floor();
            if steep {
                self.plot_with_intensity(
                    y_intercept.floor() as i32,
                    x_current as i32,
                    1.0 - intensity,
                );
                self.plot_with_intensity(
                    y_intercept.floor() as i32 + 1,
                    x_current as i32,
                    intensity,
                );
            } else {
                self.plot_with_intensity(
                    x_current as i32,
                    y_intercept.floor() as i32,
                    1.0 - intensity,
                );
                self.plot_with_intensity(
                    x_current as i32,
                    y_intercept.floor() as i32 + 1,
                    intensity,
                );
            }

            // Update the y-intercept
            y_intercept += gradient;
            x_current += 1.0;
        }
        println!("Points: {:?}", self.points);
    }

    fn plot_with_intensity(&mut self, x: i32, y: i32, intensity: f32) {
        let brightness = (intensity * 255.0) as u8;
        self.points
            .push((egui::Pos2::new(x as f32, y as f32), brightness));
    }

    fn draw_debug_grid(&self, ui: &mut egui::Ui) {
        let grid_size = 10.0;
        let grid_color = egui::Color32::from_gray(200);

        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

        let rect = response.rect;

        let mut window_center = rect.left_top();

        // println!("Drawing: {}", self.debug_points.len());
        for (point, brightness) in &self.debug_points {
            let grid_point = egui::Pos2::new(
                (point.x * grid_size) + window_center.x,
                (point.y * grid_size) + window_center.y,
            );
            let color = egui::Color32::from_rgba_premultiplied(0, 0, 0, *brightness);
            let rect = egui::Rect::from_min_size(grid_point, egui::Vec2::new(1.0, 1.0) * grid_size);
            painter.rect_filled(rect, 0.0, color);
        }
        // Draw grid
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
