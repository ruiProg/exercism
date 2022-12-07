use std::collections::HashMap;

pub type Attributes = HashMap<String, String>;
pub type AttributesInput<'a> = [(&'a str, &'a str)];

pub trait WithAttributes {
    fn attributes(&mut self) -> &mut Attributes;

    fn push_attrs(mut self, input: &AttributesInput<'_>) -> Self
    where
        Self: Sized,
    {
        self.attributes().extend(
            input
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string())),
        );
        self
    }
}
