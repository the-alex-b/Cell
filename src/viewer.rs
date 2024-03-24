use crate::spreadsheet::Spreadsheet;
use prettytable::{Cell as DisplayCell, Row, Table};

pub struct Viewer {
    pub viewport_x: (i32, i32),
    pub viewport_y: (i32, i32),
}

impl Viewer {
    pub fn new() -> Viewer {
        Viewer {
            viewport_x: (0, 10),
            viewport_y: (0, 10),
        }
    }
    pub fn move_viewport(&mut self, dx: i32, dy: i32) {
        self.viewport_x = (self.viewport_x.0 + dx, self.viewport_x.1 + dx);
        self.viewport_y = (self.viewport_y.0 + dy, self.viewport_y.1 + dy);
    }

    pub fn display(&self, spreadsheet: &Spreadsheet) {
        let mut table = Table::new();

        for y in self.viewport_y.0..=self.viewport_y.1 {
            let mut row = Vec::new();
            for x in self.viewport_x.0..=self.viewport_x.1 {
                let key = format!("{}:{}", x, y);
                if let Some(cell) = spreadsheet.cells.get(&key) {
                    row.push(DisplayCell::new(&cell.get_value()));
                } else {
                    row.push(DisplayCell::new(""));
                }
            }
            table.add_row(Row::new(row));
        }

        table.printstd(); // Prints the table to stdout
    }
}
// New method to move the viewport
