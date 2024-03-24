use prettytable::{Cell as DisplayCell, Row, Table};
use std::collections::HashMap;

use crate::cell::{Cell, CellContent};
use crate::dependency_graph::DependencyGraph;

#[derive(Debug)]
pub struct Spreadsheet {
    pub cells: HashMap<String, Cell>,
    pub dependency_graph: DependencyGraph,
    pub viewport_x: (i32, i32),
    pub viewport_y: (i32, i32),
}

impl Spreadsheet {
    pub fn new() -> Spreadsheet {
        Spreadsheet {
            cells: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
            viewport_x: (0, 10),
            viewport_y: (0, 10),
        }
    }

    pub fn add_to_spreadsheet(&mut self, mut cell: Cell) {
        // Calculate the result
        let result = cell.cell_content.evaluate(&self.cells);
        cell.result = result;

        // --- HASHMAP entry for visualization
        self.cells.insert(cell.pk.to_string(), cell.clone());

        // --- DEPENDENCY GRAPH for relations and calculation
        if let CellContent::Formula(_) = &cell.cell_content {
            let dependencies = cell.get_dependencies(&self.cells).unwrap();
            for dep in dependencies {
                self.dependency_graph
                    .add_edge(cell.pk.clone(), dep.pk.clone());
            }

            // Check for cycles after adding dependencies
            if self.dependency_graph.would_create_cycle(&cell.pk) {
                panic!("Cycle detected! Panic :O");
                // Handle cycle detection, e.g., revert changes or alert the user
            }
        } else {
            self.dependency_graph.add_single_node(cell.pk)
        }
    }

    // New method to move the viewport
    pub fn move_viewport(&mut self, dx: i32, dy: i32) {
        self.viewport_x = (self.viewport_x.0 + dx, self.viewport_x.1 + dx);
        self.viewport_y = (self.viewport_y.0 + dy, self.viewport_y.1 + dy);
    }

    pub fn display(&self) {
        let mut table = Table::new();

        for y in self.viewport_y.0..=self.viewport_y.1 {
            let mut row = Vec::new();
            for x in self.viewport_x.0..=self.viewport_x.1 {
                let key = format!("{}:{}", x, y);
                if let Some(cell) = self.cells.get(&key) {
                    row.push(DisplayCell::new(&cell.get_value()));
                } else {
                    row.push(DisplayCell::new(""));
                }
            }
            table.add_row(Row::new(row));
        }

        table.printstd(); // Prints the table to stdout
    }
}
