use cpal::traits::HostTrait;
use eframe::{
    egui::{self, FontData, FontDefinitions, ScrollArea, Separator},
    epaint::FontFamily,
};

pub fn start_ui() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "_audio",
        native_options,
        Box::new(|cc| Box::new(UI::new(cc))),
    );
}

struct UI {
    logs: Vec<String>,
    host: HostTrait,
}

impl UI {
    fn default() -> Self {
        Self {
            logs: vec![],
            host: cpal::default_host(),
        }
    }

    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "jetbrains".to_owned(),
            FontData::from_static(include_bytes!("../res/JetBrainsMono-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "jetbrains".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("jetbrains".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Self::default()
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.logs.iter().map(|x| ui.label(x));
            });
            ui.add(Separator::default().vertical());
            ui.heading("Hello World!");
        });
    }
}
