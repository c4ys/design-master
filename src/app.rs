use eframe::{egui, epi};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
enum LeftPanel {
    Page,
    Component,
    Element,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left_panel: LeftPanel,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            left_panel: LeftPanel::Page,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Design Master"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        setup_custom_fonts(_ctx);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { left_panel } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("side_choose_panel")
                    .resizable(false)
                    .min_height(0.0)
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.selectable_value(left_panel, LeftPanel::Page, "??????");
                            ui.selectable_value(left_panel, LeftPanel::Component, "??????");
                            ui.selectable_value(left_panel, LeftPanel::Element, "??????");
                        });
                    });

                match left_panel {
                    LeftPanel::Page => {
                        ui.heading("????????????");
                    }
                    LeftPanel::Component => {
                        ui.heading("????????????");
                    }
                    LeftPanel::Element => {
                        ui.heading("????????????");
                    }
                }
            });

        egui::SidePanel::right("right_bar")
            .default_width(350.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("????????????");
            });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("????????????");
            ui.end_row();
            ui.label("????????????????????????");
            ui.end_row();
            ui.label("????????? -- ?????????????????????????????????/???????????????");
            ui.end_row();
            ui.label("??????????????? -- ????????????Web/??????/?????????/??????????????????");
            ui.end_row();
            ui.label("????????? -- ????????????");
            ui.end_row();
            ui.label("????????? -- ????????????????????????K???????????????G");
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../fonts/simsun.ttc")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}