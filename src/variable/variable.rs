pub struct Variable {
    domain: Vec<i32>,
    value: Option<i32>,
}

impl Variable {
    pub fn new(domain: Vec<i32>) -> Self {
        Variable {
            domain,
            value: None,
        }
    }

    pub fn set_value(&mut self, value: i32) {
            self.value = Some(value);
        if self.domain.contains(&value) {
        }
    }
    pub fn unset_value(&mut self) {
        self.value = None;
    }

    pub fn get_value(&self) -> Option<i32> {
        self.value
    }

    pub fn get_domain(&self) -> Vec<i32> {
        self.domain.clone()
    }
}
