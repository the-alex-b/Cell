use crate::cell_content::CellContent;
use crate::spreadsheet::Spreadsheet;
use std::collections::HashMap;
use std::ops::Add;

const UNSUPPORTED_STRING: &str = "!UNSUPPORTED";

#[derive(Debug, Clone)]
pub struct Cell {
    pub pk: String,
    // pub uuid: String,
    pub x: i32,
    pub y: i32,
    pub cell_content: CellContent,
    pub result: CellContent,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_content: CellContent) -> Cell {
        let pk = format!("{}:{}", x, y);
        let result: CellContent = CellContent::None;

        Cell {
            pk,
            x,
            y,
            cell_content,
            result,
        }
    }

    pub fn evaluate_cell(&mut self, cells: HashMap<String, Cell>) -> () {
        match self.clone().cell_content {
            CellContent::Formula(formula_str) => {
                // For simplicity, assume formula_str is "x:y OP x:y", e.g., "1:1 + 2:2"
                let parts: Vec<&str> = formula_str.split_whitespace().collect();
                if parts.len() == 3 {
                    let left = cells.get(parts[0]).unwrap().clone().cell_content;
                    let right = cells.get(parts[2]).unwrap().clone().cell_content;
                    let result = match parts[1] {
                        "+" => left + right,
                        // "-" => left - right,
                        // "*" => left * right,
                        // "/" => left / right,
                        _ => CellContent::Text(String::from(UNSUPPORTED_STRING)),
                    };
                    self.result = result;
                } else {
                    self.result = CellContent::Text(String::from(UNSUPPORTED_STRING))
                }
            }
            _ => self.result = self.cell_content.clone(),
        }
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, rhs: Self) -> Self::Output {
        Cell::new(self.x, self.y, self.cell_content + rhs.cell_content)
    }
}

// impl Sub for Cell {
//     type Output = Cell;

//     fn sub(self, other: Cell) -> Cell {
//         use CellContent::*;

//         match (self.cell_content, other.cell_content) {
//             (Int(a), Int(b)) => Cell::new(Int(a - b)),
//             (Float(a), Float(b)) => Cell::new(Float(a - b)),
//             (Int(a), Float(b)) => Cell::new(Float(a as f32 - b as f32)),
//             (Float(a), Int(b)) => Cell::new(Float(a as f32 - b as f32)),

//             // For Text concatenation, we treat any cell as text when combined with a Text variant.
//             (Text(_), _) | (_, Text(_)) => Cell::new(Text(UNSUPPORTED_STRING.to_string())),

//             // Catch all
//             _ => Cell::new(Text(UNSUPPORTED_STRING.to_string())),
//         }
//     }
// }

// impl Div for Cell {
//     type Output = Cell;

//     fn div(self, other: Cell) -> Cell {
//         use CellContent::*;

//         match (self.cell_content, other.cell_content) {
//             (Int(a), Int(b)) => Cell::new(Int(a / b)),
//             (Float(a), Float(b)) => Cell::new(Float(a / b)),
//             (Int(a), Float(b)) => Cell::new(Float(a as f32 / b as f32)),
//             (Float(a), Int(b)) => Cell::new(Float(a as f32 / b as f32)),

//             // For Text concatenation, we treat any cell as text when combined with a Text variant.
//             (Text(_), _) | (_, Text(_)) => Cell::new(Text(UNSUPPORTED_STRING.to_string())),

//             // Catch all
//             _ => Cell::new(Text(UNSUPPORTED_STRING.to_string())),
//         }
//     }
// }

// impl Mul for Cell {
//     type Output = Cell;

//     fn mul(self, other: Cell) -> Cell {
//         use CellContent::*;

//         match (self.cell_content, other.cell_content) {
//             (Int(a), Int(b)) => Cell::new(Int(a * b)),
//             (Float(a), Float(b)) => Cell::new(Float(a * b)),
//             (Int(a), Float(b)) => Cell::new(Float(a as f32 * b as f32)),
//             (Float(a), Int(b)) => Cell::new(Float(a as f32 * b as f32)),

//             // For Text concatenation, we treat any cell as text when combined with a Text variant.
//             (Text(_), _) | (_, Text(_)) => Cell::new(Text(UNSUPPORTED_STRING.to_string())),

//             // Catch all
//             _ => Cell::new(Text(UNSUPPORTED_STRING.to_string())),
//         }
//     }
// }
