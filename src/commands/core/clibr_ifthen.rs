#[derive(Default)]
pub struct IfThen<T: Default> {
    condition: bool,
    true_value: Option<T>,
    false_value: Option<T>,
}

impl<T: Default> IfThen<T> {
    pub fn new() -> Self {
        IfThen {
            condition: false,
            true_value: None,
            false_value: None,
        }
    }

    pub fn when(mut self, condition: bool) -> Self {
        self.condition = condition;
        self
    }

    pub fn then(mut self, value: T) -> Self {
        self.true_value = Some(value);
        self
    }

    pub fn else_or(mut self, value: T) -> Self {
        self.false_value = Some(value);
        self
    }

    pub fn evaluate(self) -> T {
        if self.condition {
            self.true_value.unwrap_or_default()
        } else {
            self.false_value.unwrap_or_default()
        }
    }
}
