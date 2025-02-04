#[allow(dead_code)]
mod pixel;
mod app;
mod second_order_lines;
mod lines;
use app::PaintApp;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "GIIS",
        options,
        Box::new(|_cc| Ok(Box::new(PaintApp::default()))),
    )
}

