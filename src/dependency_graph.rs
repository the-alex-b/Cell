use std::collections::{HashMap, HashSet};

// Assuming CellId is a type that uniquely identifies a cell.
type CellId = String;

pub struct DependencyGraph {
    edges: HashMap<CellId, Vec<CellId>>,
}

impl DependencyGraph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_dependency(&mut self, from: CellId, to: CellId) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
    }

    // Checks if adding a dependency would create a cycle.
    fn would_create_cycle(&self, start: &CellId) -> bool {
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

            if let Some(dependencies) = self.edges.get(node) {
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
}
