use prettytable::{Cell as DisplayCell, Row, Table};
use std::collections::HashMap;

mod cell;
use crate::cell::{Cell, CellContent};

#[derive(Debug)]
struct Spreadsheet {
    cells: HashMap<String, Cell>,
    viewport_x: (i32, i32),
    viewport_y: (i32, i32),
}

impl Spreadsheet {
    fn new() -> Spreadsheet {
        Spreadsheet {
            cells: HashMap::new(),
            viewport_x: (0, 10),
            viewport_y: (0, 10),
        }
    }

    fn insert(&mut self, cell: Cell) {
        self.cells.insert(cell.pk.to_string(), cell);
    }

    // New method to move the viewport
    fn move_viewport(&mut self, dx: i32, dy: i32) {
        self.viewport_x = (self.viewport_x.0 + dx, self.viewport_x.1 + dx);
        self.viewport_y = (self.viewport_y.0 + dy, self.viewport_y.1 + dy);
    }

    fn display(&self) {
        let mut table = Table::new();

        for y in self.viewport_y.0..=self.viewport_y.1 {
            let mut row = Vec::new();
            for x in self.viewport_x.0..=self.viewport_x.1 {
                let key = format!("{}:{}", x, y);
                if let Some(cell) = self.cells.get(&key) {
                    row.push(DisplayCell::new(
                        &cell.get_value(&self.cells).to_display_string(),
                    ));
                } else {
                    row.push(DisplayCell::new(""));
                }
            }
            table.add_row(Row::new(row));
        }

        table.printstd(); // Prints the table to stdout
    }
}

fn main() {
    let res = CellContent::Text("asdas".to_string()) + CellContent::Integer(5);

    dbg!(res);

    let mut sheet = Spreadsheet::new();

    let cell = Cell::new(0, 0, CellContent::Float(12.3));
    sheet.insert(cell);

    let cell = Cell::new(1, 1, CellContent::Integer(2));
    sheet.insert(cell);

    let cell = Cell::new(2, 2, CellContent::Text("Hello World".to_string()));
    sheet.insert(cell);

    let cell = Cell::new(3, 3, CellContent::Formula("0:0 + 1:1".to_string()));
    sheet.insert(cell);

    let cell = Cell::new(4, 4, CellContent::Formula("3:3 + 1:1".to_string()));
    sheet.insert(cell);

    // loop {
    //     println!(
    //         "Viewport is at x: {:?}, y: {:?}",
    //         sheet.viewport_x, sheet.viewport_y
    //     );
    //     sheet.display();
    //     println!("Use WASD to move the viewport, or Q to quit:");

    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     match input.trim() {
    //         "w" | "W" => {
    //             println!("W hit!");
    //             sheet.move_viewport(0, -1)
    //         }
    //         "s" | "S" => sheet.move_viewport(0, 1),
    //         "a" | "A" => sheet.move_viewport(-1, 0),
    //         "d" | "D" => sheet.move_viewport(1, 0),
    //         "q" | "Q" => break,
    //         _ => println!("Invalid input, please use W, A, S, D to navigate, or Q to quit."),
    //     }

    //     // Clear the console. This command works on most UNIX-like systems.
    //     // Windows users might see odd behavior.
    //     print!("\x1B[2J\x1B[1;1H");
    // }
    sheet.display();
}
