use crate::cell_content::CellContent;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

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

        // Evaluate cell to determine display value, if static just write to string
    }

    pub fn get_dependencies(&self, cells: &HashMap<String, Cell>) -> Result<Vec<Cell>, String> {
        match self.cell_content.clone() {
            CellContent::Formula(formula_str) => {
                println!("Finding dependencies");
                let parts: Vec<&str> = formula_str.split_whitespace().collect();
                let left = cells.get(parts[0]).unwrap();

                let right = cells.get(parts[2]).unwrap();

                let mut dependencies = Vec::new();
                dependencies.push(left.clone());
                dependencies.push(right.clone());
                Ok(dependencies)
            }
            _ => Err("Can't parse formula".to_string()),
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
