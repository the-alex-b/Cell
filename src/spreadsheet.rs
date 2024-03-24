use std::collections::HashMap;

use crate::cell::Cell;
use crate::dependency_graph::DependencyGraph;

use crate::cell_content::CellContent;

#[derive(Debug)]
pub struct Spreadsheet {
    pub cells: HashMap<String, Cell>,
    pub dependency_graph: DependencyGraph,
}

impl Spreadsheet {
    pub fn new() -> Spreadsheet {
        Spreadsheet {
            cells: HashMap::new(),
            dependency_graph: DependencyGraph::new(),
        }
    }

    pub fn add_to_spreadsheet(&mut self, mut cell: Cell) {
        // TODO: combine evaluation step here with get_dependencies for adding to dependency graph.
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
}
