use rustine::core::solver::Solver;
use rustine::core::variable::Variable;
use rustine::core::constraint::InequalityConstraint;

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
