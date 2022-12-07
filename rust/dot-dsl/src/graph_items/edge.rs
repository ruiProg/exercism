use crate::attr::{Attributes, AttributesInput, WithAttributes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge<'a> {
    source: &'a str,
    destination: &'a str,
    attrs: Attributes,
}

impl<'a> Edge<'a> {
    pub fn new(source: &'a str, destination: &'a str) -> Self {
        Self {
            source,
            destination,
            attrs: Attributes::new(),
        }
    }

    pub fn with_attrs(self, attrs: &AttributesInput<'a>) -> Self {
        self.push_attrs(attrs)
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|element| &element[..])
    }
}

impl WithAttributes for Edge<'_> {
    fn attributes(&mut self) -> &mut Attributes {
        &mut self.attrs
    }
}
