use egui::{FontDefinitions, FontFamily, FontData};

pub fn setup_custom_fonts(ctx: &egui::Context) {

    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Minecraft".to_owned(),
        FontData::from_static(include_bytes!("../assets/font/Minecraft.ttf")).into(),
    );

   fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "Minecraft".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "Minecraft".to_owned());

    ctx.set_fonts(fonts);
}
