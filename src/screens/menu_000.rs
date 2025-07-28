use crate::application::app::MyApp;
use egui::{RichText, FontId, FontFamily, Layout, Align};

impl MyApp {
    pub fn mostrar_menu(&mut self, ctx: &egui::Context) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(20.0); ui.label(RichText::new("¡Elige uno!").font(FontId::new(30.0, FontFamily::Proportional)),
                );
            });

            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let fondo = egui::include_image!("../../assets/img/fondo_general.jpg");
                ui.image(fondo);
            });

        });

    }
}
