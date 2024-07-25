use std::collections::HashMap;

use crate::Constraint;
use crate::InequalityConstraint;
use crate::Variable;

pub struct Solver {
    variables: HashMap<String, Variable>,
    constraints: Vec<InequalityConstraint>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            variables: HashMap::new(),
            constraints: vec![],
        }
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.insert(variable.get_name().to_owned() , variable);
    }

    pub fn add_constraint(&mut self, constraint: InequalityConstraint) {
        // check if variables already exists
        for var_name in constraint.get_variables().iter() {
            match self.variables.get(var_name) {
                None => {panic!("Variable of the constraint are not known by the solver")},
                _ => {..}
            };

        }
        self.constraints.push(constraint);
    }

    pub fn variables(&self) -> &HashMap<String, Variable> {
        &self.variables
    }

    pub fn constraints(&self) -> &Vec<InequalityConstraint> {
        &self.constraints
    }

    pub fn solve(&self) -> Option<HashMap<String, i32>> {
        // let mut values = vec![];
        let mut values: HashMap<String, i32>= HashMap::new();
        self.solve_recursive(&mut values)
    }

    fn solve_recursive(&self, values: &mut HashMap<String, i32>) -> Option<HashMap<String, i32>> {
        if values.len() == self.variables.len() {
            if self.satisfies_constraints(values) {
                return Some(values.clone());
            } else {
                return None;
            }
        }
        let mut index: String = "".to_string();
        'keyloop: for test_key in self.variables.clone().into_keys().collect::<Vec<String>>().iter() {
            if values.get(test_key) == None {
                    index = test_key.to_owned();
                    break 'keyloop;
            }
        }

        for value in &self.variables.get(&index)?.get_domain() {
            values.insert(index.to_owned(), *value);
            if let Some(solution) = self.solve_recursive(values) {
                return Some(solution);
            }

            values.remove(&index);
        }

        None
    }

    fn satisfies_constraints(&self, values: &HashMap<String, i32>) -> bool {
        for constraint in &self.constraints {
            if !constraint.evaluate(values) {
                return false;
            }
        }
        true
    }
}
