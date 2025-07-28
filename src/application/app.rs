use egui::{RichText, FontId, FontFamily};
use std::{cell::Cell, rc::Rc};

pub struct MyApp {
    _value: Rc<Cell<u32>>,
    _spin: bool,
    _blinky: bool,

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            _value: Rc::new(Cell::new(42)),
            _spin: false,
            _blinky: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Por cambiar 
            
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(RichText::new("¡Elige uno!").font(FontId::new(30.0, FontFamily::Proportional)),);
            });

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.image(egui::include_image!("../../assets/img/fondo_general.jpg"));
            });

        });
    
    
    
    }

}