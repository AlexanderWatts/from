#[derive(Debug)]
pub struct IdGenerator {
    id: usize,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn generate(&mut self) -> usize {
        self.id += 1;
        self.id
    }
}
