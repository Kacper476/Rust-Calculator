// src/app.rs
use eframe::egui;

pub struct MyApp {
    pub counter: f32, // Używamy typu f32 dla wartości zmiennoprzecinkowych
}

impl Default for MyApp {
    fn default() -> Self {
        Self { counter: 2137.0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(250.0);
                ui_counter(ui, &mut self.counter);
                //ui_counter(ui, &mut self.counter);
            });
        });





        
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut f32) {
    ui.horizontal(|ui| {
        ui.label(format!("Counter: {:.1}", counter));

        ui.add(egui::Slider::new(counter, 0.0..=10000.0)
            .text("Counter")
            .clamp_to_range(true)
            .step_by(1.1)
            .show_value(false)); 

        
    });

    ui.vertical(|ui| {
        ui.add_space(120.0);
        ui.label(format!("Counter: {:.1}", counter));

        ui.add(egui::Slider::new(counter, 0.0..=10000.0)
            .text("Counter")
            .clamp_to_range(true)
            .step_by(1.1)
            .show_value(false)); 

            ui.label(format!("Counter: {:.1}", counter));
    });

}
