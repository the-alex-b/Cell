use crate::cell::Cell;
use crate::cell_content::CellContent;
use crate::spreadsheet::Spreadsheet;
use eframe::egui::{self};

impl eframe::App for Spreadsheet {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cell");
            if ui.button("Add cell").clicked() {
                let cell = Cell::new(0, 0, CellContent::Integer(5));
                self.add_to_spreadsheet(cell);

                let cell = Cell::new(1, 0, CellContent::Integer(5));
                self.add_to_spreadsheet(cell);

                let cell = Cell::new(2, 0, CellContent::Integer(8));
                self.add_to_spreadsheet(cell);

                let cell = Cell::new(1, 1, CellContent::Formula("1:0 + 2:0".to_string()));
                self.add_to_spreadsheet(cell);

                let cell = Cell::new(2, 2, CellContent::Formula("1:1 + 2:0".to_string()));
                self.add_to_spreadsheet(cell);
            };

            let max_rows = 10;
            let max_cols = 10;

            egui::Grid::new("Grid").striped(true).show(ui, |ui| {
                for row in 0..max_rows {
                    for col in 0..max_cols {
                        let cell_id = format!("{}:{}", col, row);
                        if let Some(cell) = self.cells.get_mut(&cell_id) {
                            let response = ui.text_edit_singleline(
                                &mut cell.result.to_display_string().to_owned(),
                            );

                        //
                        //     // Here you can handle the response, e.g., to update cell content
                        } else {
                            // Placeholder for cells not in the map
                            ui.text_edit_singleline(&mut ("").to_owned());
                        }
                    }
                    ui.end_row()
                }
                ui.end_row();
            });
        });
    }
}
