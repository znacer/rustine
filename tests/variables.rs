// use std::collections::HashSet// tests/variable.rs
use rustine::core::Variable;

#[test]
fn test_create_variable() {
    let domain = vec![1, 2, 3];
    let variable = Variable::new(domain.clone());
    assert_eq!(variable.get_domain(), domain);
    assert_eq!(variable.get_value(), None);
}

#[test]
fn test_set_value() {
    let domain = vec![1, 2, 3];
    let mut variable = Variable::new(domain.clone());
    variable.set_value(2);
    assert_eq!(variable.get_value(), Some(2));
}

#[test]
fn test_set_value_out_of_domain() {
    let domain = vec![1, 2, 3];
    let mut variable = Variable::new(domain.clone());
    variable.set_value(4);
    assert_eq!(variable.get_value(), None);
}

#[test]
fn test_get_value() {
    let domain = vec![1, 2, 3];
    let mut variable = Variable::new(domain.clone());
    variable.set_value(2);
    assert_eq!(variable.get_value(), Some(2));
}

#[test]
fn test_get_domain() {
    let domain = vec![1, 2, 3];
    let variable = Variable::new(domain.clone());
    assert_eq!(variable.get_domain(), domain);
}
