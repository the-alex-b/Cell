mod application;
mod cell;
mod cell_content;
mod dependency_graph;
mod parser;
mod spreadsheet;
mod viewer;

use crate::application::Application;

fn main() {
    let application = Application::new();
    application.start()

    // let mut spreadsheet = Spreadsheet::new();

    // let cell = Cell::new(0, 0, CellContent::Integer(5));
    // spreadsheet.add_to_spreadsheet(cell);

    // let cell = Cell::new(2, 0, CellContent::Integer(8));
    // sheet.add_to_spreadsheet(cell);

    // let cell = Cell::new(1, 1, CellContent::Formula("1:0 + 2:0".to_string()));
    // sheet.add_to_spreadsheet(cell);

    // let cell = Cell::new(0, 2, CellContent::Formula("1:1 + 0:0".to_string()));
    // sheet.add_to_spreadsheet(cell);

    // let cell = Cell::new(2, 2, CellContent::Integer(1));
    // sheet.add_to_spreadsheet(cell);

    // let cell = Cell::new(1, 3, CellContent::Formula("0:2 + 2:2".to_string()));
    // sheet.add_to_spreadsheet(cell);

    // // dbg!(sheet.dependency_graph.clone());
    // // dbg!(sheet.dependency_graph.topological_sort().unwrap());

    // let cell = Cell::new(1, 3, CellContent::Integer(30));
    // sheet.add_to_spreadsheet(cell);
    // // viewer.display(&sheet);
}
