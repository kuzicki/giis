use super::PaintApp;
use super::{DrawingState, Mode, ParameterState, PolygonTest, ViewportSettings};
use eframe::egui::{Ui, Window};

fn show_debug(ui: &mut Ui, drawing_state: &mut DrawingState) {
    if ui
        .checkbox(&mut (drawing_state.mode == Mode::Debug), "Debug mode")
        .clicked()
    {
        if drawing_state.mode == Mode::Debug {
            drawing_state.change_mode(Mode::None);
        } else {
            drawing_state.change_mode(Mode::Debug);
        };
    }
}

fn show_curve_panel(ui: &mut Ui, drawing_state: &mut DrawingState) {
    ui.horizontal(|ui| {
        if ui
            .checkbox(
                &mut (drawing_state.mode == Mode::MoveControlPoints(None)),
                "Move points",
            )
            .clicked()
        {
            if drawing_state.mode == Mode::MoveControlPoints(None) {
                drawing_state.change_mode(Mode::None);
            } else {
                drawing_state.change_mode(Mode::MoveControlPoints(None));
            };
        }
    });
}

fn show_polygon_panel(
    ui: &mut Ui,
    drawing_state: &mut DrawingState,
    viewport: &mut ViewportSettings,
) {
    ui.horizontal(|ui| {
        if ui
            .checkbox(
                &mut (drawing_state.mode == Mode::PolygonOperations(None, PolygonTest::None)),
                "Polygon select",
            )
            .clicked()
        {
            if drawing_state.mode == Mode::PolygonOperations(None, PolygonTest::None) {
                drawing_state.change_mode(Mode::None);
            } else {
                drawing_state.change_mode(Mode::PolygonOperations(None, PolygonTest::None));
            };
        }
        if let Mode::PolygonOperations(Some(index), ..) = &mut drawing_state.mode {
            let figure = drawing_state.figures[*index]
                .as_polygon_transform_mut()
                .unwrap();
            if ui.button("Test convex").clicked() {
                let res = if figure.test_convex() {
                    "The polygon is convex".to_string()
                } else {
                    "The polygon is not convex".to_string()
                };
                viewport.modal_window_text = res;
            }
            if ui.button("Find internal normals").clicked() {
                figure.find_internal_normals();
            }
            if ui.button("Graham").clicked() {
                figure.graham();
            }
            if ui.button("Jarvis").clicked() {
                figure.jarvis();
            }
            if ui.button("Test dot").clicked() {
                if let Mode::PolygonOperations(index, PolygonTest::None) = &mut drawing_state.mode {
                    drawing_state.mode = Mode::PolygonOperations(*index, PolygonTest::Dot);
                }
            }
            if ui.button("Test line").clicked() {
                if let Mode::PolygonOperations(index, PolygonTest::None) = &mut drawing_state.mode {
                    drawing_state.mode = Mode::PolygonOperations(*index, PolygonTest::Line(None));
                }
            }
            if ui.button("I").clicked() {
                figure.first();
            }
            if ui.button("II").clicked() {
                figure.second();
            }
            if ui.button("III").clicked() {
                figure.third();
            }
            if ui.button("IV").clicked() {
                figure.fourth();
            }
            if ui.button("reset fill").clicked() {
                figure.reset_fill();
            }
        }
    });
}

impl PaintApp {
    pub(super) fn show_panel(&mut self, ui: &mut Ui) {
        use ParameterState as ps;
        match self.drawing.parameters {
            ps::Line(..)
            | ps::Circle(..)
            | ps::Ellips(..)
            | ps::Parabola(..)
            | ps::Hyperbola(..) => show_debug(ui, &mut self.drawing),
            ps::Curve(..) => show_curve_panel(ui, &mut self.drawing),
            ps::Object(..) | ps::Voronoi(..) | ps::Delone(..) => (),
            ps::Polygon(..) => show_polygon_panel(ui, &mut self.drawing, &mut self.viewport),
        }
    }
}
