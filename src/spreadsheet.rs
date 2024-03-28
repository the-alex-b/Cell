use std::collections::HashMap;

use crate::cell::Cell;
use crate::dependency_graph::DependencyGraph;

use crate::cell_content::CellContent;
use crate::parser::get_dependencies;

#[derive(Debug, Clone)]
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

    pub fn add_to_spreadsheet(&mut self, cell: Cell) {
        // --- HASHMAP entry for visualization
        self.cells.insert(cell.clone().pk.to_string(), cell.clone());

        // --- DEPENDENCY GRAPH for relations and calculation
        if let CellContent::Formula(_) = &cell.cell_content {
            let dependencies = get_dependencies(cell.clone(), &self).unwrap();
            for dep in dependencies {
                self.dependency_graph
                    .add_edge(cell.pk.clone(), dep.pk.clone());
            }

            // Check for cycles after adding dependencies
            if self.dependency_graph.would_create_cycle(&cell.clone().pk) {
                panic!("Cycle detected! PANIC!!");
                // Handle cycle detection, e.g., revert changes or alert the user
            }
        } else {
            self.dependency_graph.add_single_node(cell.clone().pk)
        }
        dbg!(cell.pk.clone());
        self.evaluate_affected_cells(cell.pk);
    }

    fn evaluate_affected_cells(&mut self, pk: String) -> () {
        println!("Evaluate affected cells {}", pk.clone());
        let filtered_ids = self.dependency_graph.get_affected_cells(&pk);

        dbg!(filtered_ids.clone());
        for pk in filtered_ids {
            // We clone here so we can pass it later to the evaluate cells method
            let spreadsheet_cells = self.cells.clone();

            // Get the c
            let cell: &mut Cell = self.cells.get_mut(&pk).unwrap();
            cell.evaluate_cell(spreadsheet_cells);

            // dbg!(cell);
        }
    }

    fn re_evaluate_all_cells(&mut self) {
        // let ordered_ids: Vec<String> = self.dependency_graph.topological_sort().unwrap(); // TODO: Get only affected ids instead of all ordered ids.
        // // dbg!(ordered_ids.clone());
        todo!();
    }
}
