#[cfg(test)]
mod constraint {

    use std::collections::HashMap;

    use rustine::{Constraint, InequalityConstraint};

    #[test]
    fn test_create_inequality_constraint() {
        let constraint =
            InequalityConstraint::new(vec!["x".to_string(), "y".to_string()], |values| {
                values.get("x").unwrap() + values.get("y").unwrap() <= 10
            });
        let mut values: HashMap<String, i32> = HashMap::new();
        values.insert("x".to_string(), 5);
        values.insert("y".to_string(), 5);
        assert!(constraint.evaluate(&values));
        values.insert("y".to_string(), 6);
        assert!(!constraint.evaluate(&values));
    }

    #[test]
    fn test_inequality_constraint_with_multiple_values() {
        let constraint = InequalityConstraint::new(
            vec!["x".to_string(), "y".to_string(), "z".to_string()],
            |values| {
                values.get("x").unwrap() + values.get("y").unwrap() + values.get("z").unwrap() <= 15
            },
        );
        let mut values: HashMap<String, i32> = HashMap::new();
        values.insert("x".to_string(), 5);
        values.insert("y".to_string(), 5);
        values.insert("z".to_string(), 5);
        assert!(constraint.evaluate(&values));
        values.insert("z".to_string(), 6);
        assert!(!constraint.evaluate(&values));
    }
}
