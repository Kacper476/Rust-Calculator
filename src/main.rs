// src/main.rs
use eframe::egui;
mod app; // Importujemy moduł app

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)), 
        ..Default::default()
    };
    eframe::run_native(
        "Display Counter App",
        options,
        Box::new(|_cc| Box::<app::MyApp>::default()), 
        
    )
}
