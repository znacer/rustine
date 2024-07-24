// src/solver.rs
use crate::core::Variable;
use crate::core::Constraint;
use crate::core::InequalityConstraint;

pub struct Solver {
    variables: Vec<Variable>,
    constraints: Vec<InequalityConstraint>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            variables: vec![],
            constraints: vec![],
        }
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable);
    }

    pub fn add_constraint(&mut self, constraint: InequalityConstraint) {
        self.constraints.push(constraint);
    }

    pub fn variables(&self) -> &Vec<Variable> {
        &self.variables
    }

    pub fn constraints(&self) -> &Vec<InequalityConstraint> {
        &self.constraints
    }

    pub fn solve(&self) -> Option<Vec<i32>> {
    let mut values = vec![];
    self.solve_recursive(&mut values, 0)
}

fn solve_recursive(&self, values: &mut Vec<i32>, index: usize) -> Option<Vec<i32>> {
    if index == self.variables.len() {
        if self.satisfies_constraints(values) {
            return Some(values.clone());
        } else {
            return None;
        }
    }

    for value in &self.variables[index].get_domain() {
        values.push(*value);
        if let Some(solution) = self.solve_recursive(values, index + 1) {
            return Some(solution);
        }
        values.pop();
    }

    None
}

fn satisfies_constraints(&self, values: &Vec<i32>) -> bool {
    for constraint in &self.constraints {
        if!constraint.evaluate(values) {
            return false;
        }
    }
    true
}}
