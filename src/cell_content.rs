use crate::cell::Cell;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

const UNSUPPORTED_STRING: &str = "!UNSUPPORTED";
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

    // Add a method to evaluate the content, possibly requiring context like a spreadsheet or cell collection  TODO: cache?
    pub fn evaluate(&self, cells: &HashMap<String, Cell>) -> CellContent {
        println!("Cell is evaluated");
        match self {
            CellContent::Formula(formula_str) => {
                println!("Cell is calculated");
                // For simplicity, assume formula_str is "x:y OP x:y", e.g., "1:1 + 2:2"
                let parts: Vec<&str> = formula_str.split_whitespace().collect();
                if parts.len() == 3 {
                    // Simple validation
                    let left = cells
                        .get(parts[0])
                        .unwrap()
                        .cell_content
                        .clone()
                        .evaluate(cells);
                    let right = cells
                        .get(parts[2])
                        .unwrap()
                        .cell_content
                        .clone()
                        .evaluate(cells);
                    match parts[1] {
                        "+" => left + right,
                        // "-" => left - right,
                        // "*" => left * right,
                        // "/" => left / right,
                        _ => CellContent::Text(String::from(UNSUPPORTED_STRING)),
                    }
                } else {
                    CellContent::Text(String::from(UNSUPPORTED_STRING))
                }
            }
            _ => self.clone(),
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
