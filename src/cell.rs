use std::ops::{Add, Div, Mul, Sub};

const UNSUPPORTED_STRING: &str = "!UNSUPPORTED";

#[derive(Debug, Clone)]
pub enum CellContent {
    Text(String),
    Integer(i32),
    Float(f32),
    None,
}

impl CellContent {
    pub fn to_display_string(&self) -> String {
        match &self {
            CellContent::Integer(value) => value.to_string(),
            CellContent::Float(value) => format!("{:.2}", value),
            CellContent::Text(value) => String::from(value),

            // If the cell is a formula we need to evaluate the formula and return a string representation of the result
            // CellContent::Formula(value) => String::from(value),
            CellContent::None => String::from(""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub pk: String,
    pub x: i32,
    pub y: i32,
    pub cell_content: CellContent,
}

impl Cell {
    pub fn new(x: i32, y: i32, cell_content: CellContent) -> Cell {
        let pk = format!("{}:{}", x, y);
        Cell {
            pk,
            x,
            y,
            cell_content,
        }
    }
}

impl Add for CellContent {
    type Output = CellContent;

    fn add(self, other: CellContent) -> CellContent {
        use CellContent::*;

        match (self, other) {
            (Integer(a), Integer(b)) => Integer(a + b),
            (Float(a), Float(b)) => Float(a + b),
            (Integer(a), Float(b)) => Float(a as f32 + b as f32),
            (Float(a), Integer(b)) => Float(a as f32 + b as f32),

            // For Text concatenation, we treat any cell as text when combined with a Text variant.
            (Text(a), b) | (b, Text(a)) => Text(format!("{}{}", a, b.to_display_string())),

            // Catch all
            _ => Text(UNSUPPORTED_STRING.to_string()),
        }
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
