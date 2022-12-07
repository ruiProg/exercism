pub mod attr;
pub mod graph_items;

pub mod graph {
    pub use super::graph_items;

    use crate::attr::{Attributes, AttributesInput, WithAttributes};

    use self::graph_items::{edge::Edge, node::Node};

    #[derive(Default)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: Attributes,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(self, attrs: &AttributesInput<'a>) -> Self {
            self.push_attrs(attrs)
        }

        pub fn node(&self, node_name: &str) -> Option<&Node<'a>> {
            self.nodes.iter().find(|node| node.name() == node_name)
        }
    }

    impl WithAttributes for Graph<'_> {
        fn attributes(&mut self) -> &mut Attributes {
            &mut self.attrs
        }
    }
}
