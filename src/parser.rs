use crate::cell::Cell;
use crate::cell_content::CellContent;
use crate::spreadsheet::Spreadsheet;

const UNSUPPORTED_STRING: &str = "!UNSUPPORTED";

pub fn get_dependencies(cell: Cell, spreadsheet: &Spreadsheet) -> Result<Vec<Cell>, String> {
    match cell.cell_content.clone() {
        CellContent::Formula(formula_str) => {
            let parts: Vec<&str> = formula_str.split_whitespace().collect();
            let left = spreadsheet.cells.get(parts[0]).unwrap();

            let right = spreadsheet.cells.get(parts[2]).unwrap();

            let mut dependencies = Vec::new();
            dependencies.push(left.clone());
            dependencies.push(right.clone());
            Ok(dependencies)
        }
        _ => Err("Can't parse formula".to_string()),
    }
}

// pub fn evaluate_cell(cell: &mut Cell, spreadsheet: &Spreadsheet) -> () {
//     match cell.clone().cell_content {
//         CellContent::Formula(formula_str) => {
//             // For simplicity, assume formula_str is "x:y OP x:y", e.g., "1:1 + 2:2"
//             let parts: Vec<&str> = formula_str.split_whitespace().collect();
//             if parts.len() == 3 {
//                 let left = spreadsheet
//                     .cells
//                     .get(parts[0])
//                     .unwrap()
//                     .clone()
//                     .cell_content;
//                 let right = spreadsheet
//                     .cells
//                     .get(parts[2])
//                     .unwrap()
//                     .clone()
//                     .cell_content;
//                 let result = match parts[1] {
//                     "+" => left + right,
//                     // "-" => left - right,
//                     // "*" => left * right,
//                     // "/" => left / right,
//                     _ => CellContent::Text(String::from(UNSUPPORTED_STRING)),
//                 };
//                 cell.result = result;
//             } else {
//                 cell.result = CellContent::Text(String::from(UNSUPPORTED_STRING))
//             }
//         }
//         _ => cell.result = cell.cell_content.clone(),
//     }
// }
