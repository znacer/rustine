#[cfg(test)]
mod solver {
    use std::collections::HashMap;

    use rustine::InequalityConstraint;
    use rustine::Solver;
    use rustine::Variable;

    #[test]
    fn test_create_solver() {
        let solver = Solver::new();
        assert!(solver.variables().is_empty());
        assert!(solver.constraints().is_empty());
    }

    #[test]
    fn test_add_variable() {
        let mut solver = Solver::new();
        let domain = vec![1, 2, 3];
        let variable = Variable::new("x", domain.clone());
        solver.add_variable(variable);
        assert_eq!(solver.variables().len(), 1);
    }

    #[test]
    fn test_add_constraint() {
        let mut solver = Solver::new();
        let variable1 = Variable::new("x", vec![1, 2, 3]);
        let variable2 = Variable::new("y", vec![4, 5, 6]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        let constraint =
            InequalityConstraint::new(vec!["x".to_string(), "y".to_string()], |values| {
                values.get("x").unwrap() + values.get("y").unwrap() <= 10
            });
        solver.add_constraint(constraint);
        assert_eq!(solver.constraints().len(), 1);
    }

    #[test]
    fn test_solve() {
        let mut solver = Solver::new();
        let variable1 = Variable::new("x", vec![1, 2, 3]);
        let variable2 = Variable::new("y", vec![4, 5, 6]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        let constraint =
            InequalityConstraint::new(vec!["x".to_string(), "y".to_string()], |values| {
                values.get("x").unwrap() + values.get("y").unwrap() <= 10
            });
        solver.add_constraint(constraint);
        let solution = solver.solve();
        assert!(solution.is_some());
    }

    #[test]
    fn test_solve_no_solution() {
        let mut solver = Solver::new();
        let variable1 = Variable::new("x", vec![1, 2, 3]);
        let variable2 = Variable::new("y", vec![7, 8, 9]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        let constraint =
            InequalityConstraint::new(vec!["x".to_string(), "y".to_string()], |values| {
                values.get("x").unwrap() + values.get("y").unwrap() <= 0
            });
        solver.add_constraint(constraint);
        let solution = solver.solve();

        assert!(solution.is_none());
    }

    #[test]
    fn test_2constraint_problem() {
        let mut solver = Solver::new();
        let variable1 = Variable::new("x", vec![1, 2, 3]);
        let variable2 = Variable::new("y", vec![4, 5, 6]);
        let variable3 = Variable::new("z", vec![2, 4, 6]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        solver.add_variable(variable3);
        let constraint1 =
            InequalityConstraint::new(vec!["x".to_string(), "z".to_string()], |values| {
                2 * values.get("x").unwrap() + values.get("z").unwrap() <= 4
            });
        let constraint2 = InequalityConstraint::new(vec!["y".to_string()], |values| {
            values.get("y").unwrap() - 4 <= 0
        });

        solver.add_constraint(constraint1);
        solver.add_constraint(constraint2);
        let solution = solver.solve();

        let mut expected_solution = HashMap::new();
        expected_solution.insert("x".to_string(), 1);
        expected_solution.insert("y".to_string(), 4);
        expected_solution.insert("z".to_string(), 2);

        assert_eq!(solution, Some(expected_solution));
    }
}
