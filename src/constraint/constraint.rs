use std::collections::HashMap;

pub trait Constraint {
    fn evaluate(&self, values: &HashMap<String, i32>) -> bool;
}

pub struct InequalityConstraint {
    variables: Vec<String>,
    func: Box<dyn Fn(&HashMap<String, i32>) -> bool>,
}

impl InequalityConstraint {
    pub fn new<F>(variables: Vec<String>, func: F) -> Self
    where
        F: Fn(&HashMap<String, i32>) -> bool +'static,
    {
        InequalityConstraint { variables, func: Box::new(func) }
    }

    pub fn get_variables(&self) -> &Vec<String> {
        &self.variables
    }
}

impl Constraint for InequalityConstraint {
    fn evaluate(&self, values: &HashMap<String, i32>) -> bool {
        assert!(self.variables.len() <= values.len());
        (self.func)(values)
    }
}
