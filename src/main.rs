mod cell;
use crate::cell::Cell;
mod dependency_graph;

mod spreadsheet;
use crate::spreadsheet::Spreadsheet;

mod cell_content;
use crate::cell_content::CellContent;

mod viewer;
use crate::viewer::Viewer;

fn main() {
    let mut sheet = Spreadsheet::new();
    let mut viewer = Viewer::new();

    let cell = Cell::new(0, 0, CellContent::Integer(2));
    sheet.add_to_spreadsheet(cell);
    let cell = Cell::new(1, 0, CellContent::Integer(5));
    sheet.add_to_spreadsheet(cell);
    let cell = Cell::new(2, 0, CellContent::Integer(8));
    sheet.add_to_spreadsheet(cell);

    let cell = Cell::new(1, 1, CellContent::Formula("1:0 + 2:0".to_string()));
    sheet.add_to_spreadsheet(cell);

    let cell = Cell::new(0, 2, CellContent::Formula("1:1 + 0:0".to_string()));
    sheet.add_to_spreadsheet(cell);

    let cell = Cell::new(2, 2, CellContent::Integer(1));
    sheet.add_to_spreadsheet(cell);

    let cell = Cell::new(1, 3, CellContent::Formula("0:2 + 2:2".to_string()));
    sheet.add_to_spreadsheet(cell);

    // sheet.display();

    dbg!(sheet.dependency_graph.clone());
    dbg!(sheet.dependency_graph.topological_sort().unwrap());

    loop {
        println!(
            "Viewport is at x: {:?}, y: {:?}",
            viewer.viewport_x, viewer.viewport_y
        );
        viewer.display(&sheet);
        println!("Use WASD to move the viewport, or Q to quit:");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "w" | "W" => {
                println!("W hit!");
                viewer.move_viewport(0, -1)
            }
            "s" | "S" => viewer.move_viewport(0, 1),
            "a" | "A" => viewer.move_viewport(-1, 0),
            "d" | "D" => viewer.move_viewport(1, 0),
            "q" | "Q" => break,
            _ => println!("Invalid input, please use W, A, S, D to navigate, or Q to quit."),
        }

        // Clear the console. This command works on most UNIX-like systems.
        // Windows users might see odd behavior.
        print!("\x1B[2J\x1B[1;1H");
    }
}
