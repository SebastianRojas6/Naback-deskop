mod app;
mod font;

use font::setup_custom_fonts;
use eframe::egui;
use app::MyApp;

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 1024.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Baback-deskop",
        options,
        Box::new(|cc| {

            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}
