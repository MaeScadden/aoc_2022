pub struct Builder<'builder> {
    value: Vec<&'builder str>,
}

impl<'this> Builder<'this> {
    pub fn default() -> Self {
        Self { value: vec![] }
    }

    pub fn append(&mut self, string: &'this str) -> &mut Self {
        self.value.push(string);
        self
    }

    pub fn string(&self) -> String {
        self.value.join("")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn builds_string_as_expected() {
        let mut builder = super::Builder::default();
        let expected = "hello this is so\n cool";
        let string = builder
            .append("hello this is so")
            .append("\n")
            .append(" cool")
            .string();

        assert_eq!(expected, string)
    }
}
