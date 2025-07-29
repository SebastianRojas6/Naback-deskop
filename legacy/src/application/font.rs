use eframe::{egui,epaint::text::{FontInsert, InsertFontFamily}};

pub fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "Minecraft",
        egui::FontData::from_static(include_bytes!(
            "../../assets/font/Minecraft.ttf"
        )),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

pub fn replace_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/font/Minecraft.ttf"
        ))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Minecraft".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Minecraft".to_owned());

    ctx.set_fonts(fonts);
}


