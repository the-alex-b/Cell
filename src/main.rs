mod cell;
use crate::cell::{Cell, CellContent};
mod dependency_graph;

mod spreadsheet;
use crate::spreadsheet::Spreadsheet;

fn main() {
    let mut sheet = Spreadsheet::new();

    let cell = Cell::new(0, 0, CellContent::Float(12.3));
    sheet.add_to_spreadsheet(cell);
    // dbg!(cell.to_owned());

    let cell = Cell::new(1, 1, CellContent::Integer(2));
    sheet.add_to_spreadsheet(cell);

    // let cell = Cell::new(2, 2, CellContent::Text("Hello World".to_string()));
    // sheet.insert(cell);

    // let cell = Cell::new(3, 3, CellContent::Formula("0:0 + 1:1".to_string()));
    // sheet.insert(cell);

    let cell = Cell::new(4, 4, CellContent::Formula("0:0 + 1:1".to_string()));
    sheet.add_to_spreadsheet(cell);

    let cell = Cell::new(1, 1, CellContent::Formula("4:4 + 1:1".to_string()));
    sheet.add_to_spreadsheet(cell);

    // sheet.display();

    dbg!(sheet.dependency_graph);
}

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
