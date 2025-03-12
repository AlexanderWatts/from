pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _code_generator = CodeGenerator;
    }
}
