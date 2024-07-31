// src/app.rs
use eframe::egui;

pub struct MyApp {
    pub counter: f32, 
    pub counter2: f32,
    pub sum: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            counter: 2137.0, 
            counter2: 0.0,
            sum: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(250.0);
                ui_counter(ui, &mut self.counter, &mut self.counter2);
                ui.add_space(10.0);  
                ui_counter2(ui, &mut self.counter, &mut self.counter2);
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Add space to push elements to the edges
                ui.horizontal(|ui| {
                    if ui.add_sized(egui::vec2(100.0, 100.0), egui::Button::new("+")).clicked() {
                        // This button does nothing for now.
                    }
                    
                    ui.add_space(ui.available_width() - 100.0); // Adjust space to push the next button to the right

                    if ui.add_sized(egui::vec2(100.0, 100.0), egui::Button::new("*")).clicked() {
                        // This button does nothing for now.
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add_sized(egui::vec2(100.0, 100.0), egui::Button::new("-")).clicked() {
                    // This button does nothing for now.
                }
                ui.add_space(ui.available_width() - 100.0); // Adjust space to push the next button to the right

                if ui.add_sized(egui::vec2(100.0, 100.0), egui::Button::new("/")).clicked() {
                    // This button does nothing for now.
                }
            });
        });
    }
}

fn ui_counter(ui: &mut egui::Ui, counter: &mut f32, counter2: &mut f32) {
    ui.horizontal(|ui| {
        ui.set_min_width(100.0); 
        
        let label_width = 50.0;
        let label_height = 30.0;

        ui.allocate_ui_with_layout(
            egui::vec2(label_width, label_height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(format!("{:.1}", counter));
            }
        );

        ui.add_space(10.0); 

        ui.allocate_ui_with_layout(
            egui::vec2(label_width, label_height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.label(format!("{:.1}", counter2));
            }
        );
    });
}

fn ui_counter2(ui: &mut egui::Ui, counter: &mut f32, counter2: &mut f32) {
    ui.vertical(|ui| {
        ui.add_space(230.0);
        
        ui.horizontal(|ui| {
            ui.add_space(-120.0); 
            
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(counter, -9999.0..=9999.0)
                    .clamp_to_range(true)
                    .step_by(1.0)
                    .show_value(false)); 
                ui.add(egui::Slider::new(counter2, -9999.0..=9999.0)
                    .clamp_to_range(true)
                    .step_by(1.0)
                    .show_value(false)); 
            });
        });
    });
}
