use crate::attr::{Attributes, AttributesInput, WithAttributes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<'a> {
    name: &'a str,
    attrs: Attributes,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            attrs: Attributes::new(),
        }
    }

    pub fn with_attrs(self, attrs: &AttributesInput<'a>) -> Self {
        self.push_attrs(attrs)
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|element| &element[..])
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl WithAttributes for Node<'_> {
    fn attributes(&mut self) -> &mut Attributes {
        &mut self.attrs
    }
}
