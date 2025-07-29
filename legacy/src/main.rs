use eframe::{egui,UserEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use naback_deskop::application::font::{replace_fonts,add_font};
use naback_deskop::application::app::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1440.0, 1024.0]),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    eventloop.set_control_flow(ControlFlow::Poll);

    let mut winit_app = eframe::create_native(
        "Baback-desktop",
        options,
        Box::new(|cc| {

            // Fuentes
            add_font(&cc.egui_ctx);
            replace_fonts(&cc.egui_ctx);

            // Imágenes
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
        &eventloop,
    );

    eventloop.run_app(&mut winit_app)?;

    Ok(())
}