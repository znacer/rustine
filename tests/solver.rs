#[cfg(test)]
mod solver {
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
        let variable = Variable::new(vec![1, 2, 3]);
        solver.add_variable(variable);
        assert_eq!(solver.variables().len(), 1);
    }

    #[test]
    fn test_add_constraint() {
        let mut solver = Solver::new();
        let constraint = InequalityConstraint::new(|values| values[0] + values[1] <= 10);
        solver.add_constraint(constraint);
        assert_eq!(solver.constraints().len(), 1);
    }

    #[test]
    fn test_solve() {
        let mut solver = Solver::new();
        let variable1 = Variable::new(vec![1, 2, 3]);
        let variable2 = Variable::new(vec![4, 5, 6]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        let constraint = InequalityConstraint::new(|values| values[0] + values[1] <= 10);
        solver.add_constraint(constraint);
        let solution = solver.solve();
        assert!(solution.is_some());
    }

    #[test]
    fn test_solve_no_solution() {
        let mut solver = Solver::new();
        let variable1 = Variable::new(vec![1, 2, 3]);
        let variable2 = Variable::new(vec![7, 8, 9]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        let constraint = InequalityConstraint::new(|values| values[0] + values[1] <= 0);
        solver.add_constraint(constraint);
        let solution = solver.solve();

        assert!(solution.is_none());
    }

    #[test]
    fn test_2constraint_problem() {
        let mut solver = Solver::new();
        let variable1 = Variable::new(vec![1, 2, 3]);
        let variable2 = Variable::new(vec![4, 5, 6]);
        let variable3 = Variable::new(vec![2, 4, 6]);
        solver.add_variable(variable1);
        solver.add_variable(variable2);
        solver.add_variable(variable3);
        let constraint1 = InequalityConstraint::new(|x| 2 * x[0] + x[2] <= 4);
        let constraint2 = InequalityConstraint::new(|x| x[1] <= 4);
        solver.add_constraint(constraint1);
        solver.add_constraint(constraint2);
        let solution = solver.solve();

        assert_eq!(solution, Some(vec![1, 4, 2]));
    }
}
