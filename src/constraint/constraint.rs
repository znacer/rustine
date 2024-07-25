pub trait Constraint {
    fn evaluate(&self, values: &[i32]) -> bool;
}

pub struct InequalityConstraint {
    func: Box<dyn Fn(&[i32]) -> bool>,
}

impl InequalityConstraint {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(&[i32]) -> bool +'static,
    {
        InequalityConstraint { func: Box::new(func) }
    }
}

impl Constraint for InequalityConstraint {
    fn evaluate(&self, values: &[i32]) -> bool {
        (self.func)(values)
    }
}
