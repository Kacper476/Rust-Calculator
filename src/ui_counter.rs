use eframe::egui;

pub fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        
        if ui.button(counter.to_string()).clicked(){
            *counter += 1;
        }
        
        ui.separator();
        ui.separator();
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}
