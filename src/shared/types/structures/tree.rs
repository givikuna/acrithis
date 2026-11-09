// Majorly Assisted By: Google Gemini
// The API for the types was written by hand.
// AI was used to fill the rest of this in.
// Read AI_Policy.md

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeError {
    NotFound(String),
    Conflict(String),
    MultipleParents(String),
    Cycle { parent : String, child : String, },
}

#[derive(Debug, Clone)]
pub struct Forest<T> {
    nodes :    HashMap<String, T>,
    children : HashMap<String, Vec<String>>,
    parent :   HashMap<String, String>,
}

impl<T> Forest<T> {
    pub fn new() -> Self {
        Self { nodes :    HashMap::new(),
               children : HashMap::new(),
               parent :   HashMap::new(), }
    }

    pub fn add_node(&mut self, id : String, node : T) -> Result<(), TreeError> {
        if self.nodes.contains_key(&id) {
            return Err(TreeError::Conflict(id));
        }
        self.nodes.insert(id.clone(), node);
        self.children.insert(id, Vec::new());
        Ok(())
    }

    pub fn add_edge(&mut self, parent_id : &str, child_id : &str) -> Result<(), TreeError> {
        if !self.nodes.contains_key(parent_id) {
            return Err(TreeError::NotFound(parent_id.to_string()));
        }
        if !self.nodes.contains_key(child_id) {
            return Err(TreeError::NotFound(child_id.to_string()));
        }

        if parent_id == child_id {
            return Err(TreeError::Cycle { parent : parent_id.to_string(),
                                          child :  child_id.to_string(), });
        }

        if self.parent.contains_key(child_id) {
            return Err(TreeError::MultipleParents(child_id.to_string()));
        }

        let mut current : &str = parent_id;
        while let Some(p) = self.parent.get(current) {
            if p == child_id {
                return Err(TreeError::Cycle { parent : parent_id.to_string(),
                                              child :  child_id.to_string(), });
            }
            current = p;
        }

        self.parent
            .insert(child_id.to_string(), parent_id.to_string());
        self.children
            .get_mut(parent_id)
            .unwrap()
            .push(child_id.to_string());

        Ok(())
    }

    pub fn get_node(&self, id : &str) -> Result<&T, TreeError> {
        self.nodes
            .get(id)
            .ok_or_else(|| TreeError::NotFound(id.to_string()))
    }

    pub fn get_children(&self, id : &str) -> Result<&Vec<String>, TreeError> {
        self.children
            .get(id)
            .ok_or_else(|| TreeError::NotFound(id.to_string()))
    }

    pub fn get_parent(&self, id : &str) -> Result<Option<&String>, TreeError> {
        if !self.nodes.contains_key(id) {
            return Err(TreeError::NotFound(id.to_string()));
        }

        Ok(self.parent.get(id))
    }

    pub fn roots(&self) -> Vec<String> {
        self.nodes
            .keys()
            .filter(|id| !self.parent.contains_key(*id))
            .cloned()
            .collect()
    }

    pub fn walk_indented(&self, start_id : &str) -> Result<Vec<(String, u32)>, TreeError> {
        if !self.nodes.contains_key(start_id) {
            return Err(TreeError::NotFound(start_id.to_string()));
        }

        let mut result : Vec<(String, u32)> = Vec::new();
        let mut stack : Vec<(String, u32)> = vec![(start_id.to_string(), 0)];

        while let Some((current_id, depth)) = stack.pop() {
            result.push((current_id.clone(), depth));

            if let Some(children) = self.children.get(&current_id) {
                for child in children.iter().rev() {
                    stack.push((child.clone(), depth + 1));
                }
            }
        }

        Ok(result)
    }
}
