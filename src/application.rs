use crate::spreadsheet::Spreadsheet;
use eframe::egui;

pub struct Application {
    options: eframe::NativeOptions,
}

impl Application {
    pub fn new() -> Application {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([680.0, 420.0]),
            ..Default::default()
        };

        Application { options: options }
    }

    pub fn start(&self) -> () {
        eframe::run_native(
            "Cell",
            self.options.clone(),
            Box::new(|_cc| Box::<Spreadsheet>::default()),
        )
        .unwrap();
    }
}
