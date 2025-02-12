use super::Figure;
use super::{Mode, ParameterState};
use eframe::egui::Ui;

pub trait ModePanel {
    fn show_panel(&self, ui: &mut Ui, mode: &mut Mode, figures: &mut Vec<Box<dyn Figure>>);
}

fn show_debug(ui: &mut Ui, mode: &mut Mode, figures: &mut Vec<Box<dyn Figure>>) {
    if ui
        .checkbox(&mut (*mode == Mode::Debug), "Debug mode")
        .clicked()
    {
        if *mode == Mode::Debug {
            mode.change_to(figures, Mode::None);
        } else {
            mode.change_to(figures, Mode::Debug);
        };
    }
}

fn show_curve_panel(ui: &mut Ui, mode: &mut Mode, figures: &mut Vec<Box<dyn Figure>>) {
    ui.horizontal(|ui| {
        if ui
            .checkbox(&mut (*mode == Mode::MoveControlPoints(None)), "Move points")
            .clicked()
        {
            if *mode == Mode::MoveControlPoints(None) {
                mode.change_to(figures, Mode::None);
            } else {
                mode.change_to(figures, Mode::MoveControlPoints(None));
            };
        }
        // if ui
        //     .checkbox(&mut (*mode == Mode::ConnectCurve(None)), "Connect curves")
        //     .clicked()
        // {
        //     if *mode == Mode::ConnectCurve(None) {
        //         mode.change_to(figures, Mode::None);
        //     } else {
        //         mode.change_to(figures, Mode::ConnectCurve(None));
        //     };
        // }
    });
}

impl ModePanel for ParameterState {
    fn show_panel(&self, ui: &mut Ui, mode: &mut Mode, figures: &mut Vec<Box<dyn Figure>>) {
        use ParameterState as ps;
        match self {
            ps::Line(..)
            | ps::Circle(..)
            | ps::Ellips(..)
            | ps::Parabola(..)
            | ps::Hyperbola(..) => show_debug(ui, mode, figures),
            ps::Curve(..) => show_curve_panel(ui, mode, figures),
        }
    }
}
