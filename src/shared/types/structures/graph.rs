// Majorly Assisted By: Google Gemini
// The API for the types was written by hand.
// AI was used to fill the rest of this in.
// Read AI_Policy.md

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    MissingDependency(String),
    SelfLoop(String),
    ClosureDepthMismatch { expected:u32, got:u32, },
    Conflict(String),
    NotFound(String),
}

#[derive(Debug, Clone)]
pub struct DAG<T> {
    nodes:       HashMap<String, T>,
    depths:      HashMap<String, u32>,
    dependencies:HashMap<String, HashSet<String>>,
    dependents:  HashMap<String, HashSet<String>>,
}

impl<T> DAG<T> {
    pub fn new() -> Self {
        Self { nodes:       HashMap::new(),
               depths:      HashMap::new(),
               dependencies:HashMap::new(),
               dependents:  HashMap::new(), }
    }

    pub fn add_node(&mut self,
                    id:String,
                    node:T,
                    deps:&[String],
                    closure_depth:u32)
                    -> Result<(), GraphError> {
        if self.nodes.contains_key(&id) {
            return Err(GraphError::Conflict(id));
        }

        let mut max_dep_depth:u32 = 0;
        let mut has_deps:bool = false;

        for dep in deps {
            if dep == &id {
                return Err(GraphError::SelfLoop(id));
            }

            let dep_depth:&u32 = self.depths
                                     .get(dep)
                                     .ok_or_else(|| GraphError::MissingDependency(dep.clone()))?;

            max_dep_depth = max_dep_depth.max(*dep_depth);
            has_deps = true;
        }

        let expected_depth:u32 = if has_deps { max_dep_depth + 1 } else { 0 };
        if closure_depth != expected_depth {
            return Err(GraphError::ClosureDepthMismatch { expected:expected_depth,
                                                          got:     closure_depth, });
        }

        self.nodes.insert(id.clone(), node);
        self.depths.insert(id.clone(), closure_depth);
        self.dependencies
            .insert(id.clone(), deps.iter().cloned().collect());

        self.dependents.entry(id.clone()).or_default();

        for dep in deps {
            self.dependents
                .entry(dep.clone())
                .or_default()
                .insert(id.clone());
        }

        Ok(())
    }

    pub fn get_node(&self, id:&str) -> Result<&T, GraphError> {
        self.nodes
            .get(id)
            .ok_or_else(|| GraphError::NotFound(id.to_string()))
    }

    pub fn deps(&self, id:&str) -> Result<Vec<String>, GraphError> {
        if !self.nodes.contains_key(id) {
            return Err(GraphError::NotFound(id.to_string()));
        }

        let deps:&HashSet<String> = self.dependencies.get(id).unwrap();
        Ok(deps.iter().cloned().collect())
    }

    pub fn rdeps(&self, id:&str) -> Result<Vec<String>, GraphError> {
        if !self.nodes.contains_key(id) {
            return Err(GraphError::NotFound(id.to_string()));
        }

        let rdeps:&HashSet<String> = self.dependents.get(id).unwrap();
        Ok(rdeps.iter().cloned().collect())
    }

    pub fn transitive_closure(&self, roots:&[String]) -> Result<HashSet<String>, GraphError> {
        let mut visited:HashSet<String> = HashSet::new();
        let mut queue:VecDeque<String> = VecDeque::new();

        for root in roots {
            if !self.nodes.contains_key(root) {
                return Err(GraphError::NotFound(root.clone()));
            }
            queue.push_back(root.clone());
            visited.insert(root.clone());
        }

        while let Some(current) = queue.pop_front() {
            if let Some(deps) = self.dependencies.get(&current) {
                for dep in deps {
                    if visited.insert(dep.clone()) {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        Ok(visited)
    }
}
