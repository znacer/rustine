use rustine::core::{Constraint, InequalityConstraint};

#[test]
fn test_create_inequality_constraint() {
    let constraint = InequalityConstraint::new(|values| values[0] + values[1] <= 10);
    assert!(constraint.evaluate(&[5, 5]));
    assert!(!constraint.evaluate(&[5, 6]));
}

#[test]
fn test_evaluate_inequality_constraint() {
    let constraint = InequalityConstraint::new(|values| values[0] + values[1] <= 10);
    assert!(constraint.evaluate(&[3, 7]));
    assert!(!constraint.evaluate(&[3, 8]));
}

#[test]
fn test_inequality_constraint_with_multiple_values() {
    let constraint = InequalityConstraint::new(|values| values[0] + values[1] + values[2] <= 15);
    assert!(constraint.evaluate(&[5, 5, 5]));
    assert!(!constraint.evaluate(&[5, 5, 6]));
}
