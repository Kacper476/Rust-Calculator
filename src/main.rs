
mod app;
mod ui_counter;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Display Counter App",
        options,
        Box::new(|_cc| Box::new(app::MyApp::default())),
    )
}
