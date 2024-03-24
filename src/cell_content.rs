use std::ops::Add;

#[derive(Debug, Clone)]
pub enum CellContent {
    Text(String),
    Integer(i32),
    Float(f32),
    Formula(String),
    None,
}

impl CellContent {
    pub fn to_display_string(&self) -> String {
        match &self {
            CellContent::Integer(value) => value.to_string(),
            CellContent::Float(value) => format!("{:.2}", value),
            CellContent::Text(value) => String::from(value),
            CellContent::Formula(value) => String::from(value),

            // If the cell is a formula we need to evaluate the formula and return a string representation of the result
            // CellContent::Formula(value) => String::from(value),
            CellContent::None => String::from(""),
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
            _ => Text("Add cell_content failed".to_string()),
        }
    }
}
