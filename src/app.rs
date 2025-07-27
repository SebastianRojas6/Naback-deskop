use egui::{RichText, FontId, FontFamily};

pub struct MyApp {}


impl Default for MyApp {
    fn default() -> Self {
        Self {
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(RichText::new("¡Elige uno!").font(FontId::new(30.0, FontFamily::Proportional)),);
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
             ui.image(egui::include_image!("../assets/img/fondo_general.jpg"));
            });

        });
    
    
    
    }

}