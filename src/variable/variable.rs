#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    domain: Vec<i32>,
    value: Option<i32>,
}

impl Variable {
    pub fn new(name: &str, domain: Vec<i32>) -> Self {
        Variable {
            name: name.to_owned(),
            domain,
            value: None,
        }
    }
     pub fn get_name(&self) -> &str {
         &self.name
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
