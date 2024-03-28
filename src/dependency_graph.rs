use std::collections::{HashMap, HashSet, VecDeque};

// Assuming CellId is a type that uniquely identifies a cell.
type CellId = String;

#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    outgoing_edges: HashMap<CellId, Vec<CellId>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            outgoing_edges: HashMap::new(),
        }
    }

    pub fn add_single_node(&mut self, node: CellId) {
        self.outgoing_edges
            .entry(node.clone())
            .or_insert_with(Vec::new);
    }

    pub fn add_edge(&mut self, from: CellId, to: CellId) {
        self.outgoing_edges
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);
    }

    // Checks if adding a dependency would create a cycle.
    pub fn would_create_cycle(&self, start: &CellId) -> bool {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        self.dfs(start, &mut visited, &mut stack)
    }

    // Depth-First Search to detect cycles.
    fn dfs(
        &self,
        node: &CellId,
        visited: &mut HashSet<CellId>,
        stack: &mut HashSet<CellId>,
    ) -> bool {
        if stack.contains(node) {
            // Node is already on the stack, indicating a cycle.
            return true;
        }
        if visited.insert(node.clone()) {
            // Mark the node as visited and add it to the current path stack.
            stack.insert(node.clone());

            if let Some(dependencies) = self.outgoing_edges.get(node) {
                for dep in dependencies {
                    if self.dfs(dep, visited, stack) {
                        return true; // Cycle detected in a dependency.
                    }
                }
            }

            // Remove the node from the stack after exploring its dependencies.
            stack.remove(node);
        }
        false
    }

    // Perform a topological sort of the nodes (cells) in the graph.
    pub fn topological_sort(&self) -> Result<Vec<CellId>, String> {
        let mut in_degree = HashMap::new();
        // Initialize in-degree of all vertices as 0.
        for vertex in self.outgoing_edges.keys() {
            in_degree.entry(vertex.clone()).or_insert(0);
        }
        // Fill the in-degree of vertices.
        for deps in self.outgoing_edges.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<CellId> = VecDeque::new();
        // Enqueue all vertices with in-degree 0.
        for (vertex, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(vertex.clone());
            }
        }

        let mut visited_count = 0;
        let mut top_order = vec![];

        while let Some(vertex) = queue.pop_front() {
            top_order.push(vertex.clone());
            if let Some(deps) = self.outgoing_edges.get(&vertex) {
                for dep in deps {
                    let degree = in_degree.get_mut(dep).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dep.clone());
                    }
                }
            }
            visited_count += 1;
        }

        if visited_count != in_degree.len() {
            // There is a cycle in the graph.
            return Err("Cycle detected in the dependency graph".to_string());
        }

        Ok(top_order)
    }

    pub fn get_affected_cells(&self, updated_cell: &CellId) -> Vec<CellId> {
        let mut affected_cells = Vec::new();
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();

        to_visit.push_back(updated_cell.clone());
        visited.insert(updated_cell.clone());
        affected_cells.insert(0, updated_cell.clone());

        while let Some(cell_id) = to_visit.pop_front() {
            if let Some(dependent_cells) = self.outgoing_edges.get(&cell_id) {
                for dep_cell in dependent_cells {
                    if visited.insert(dep_cell.clone()) {
                        to_visit.push_back(dep_cell.clone());
                        affected_cells.push(dep_cell.clone());
                    }
                }
            }
        }

        affected_cells
    }
}
