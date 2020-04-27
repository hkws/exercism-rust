pub mod graph {
    use std::fmt::Debug;

    pub mod graph_items {

        pub mod edge {
            use std::collections::HashMap;
            #[derive(Default, PartialEq, Clone, Debug)]
            pub struct Edge {
                src: String,
                dst: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    let mut o = Self::default();
                    o.src = src.into();
                    o.dst = dst.into();
                    o
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = HashMap::new();
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            #[derive(Default, Clone, PartialEq, Debug)]
            pub struct Node {
                pub value: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(v: &str) -> Self {
                    let mut o = Self::default();
                    o.value = v.into();
                    o
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = HashMap::new();
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        Some(v) => Some(&v),
                        _ => None,
                    }
                }
            }
        }
    }

    use std::collections::HashMap;
    #[derive(Default, Debug)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }
    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = HashMap::new();
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }
        pub fn get_node(&self, node_value: &str) -> Option<&graph_items::node::Node> {
            for node in self.nodes.iter() {
                if node.value == node_value {
                    return Some(node);
                }
            }
            None
        }
    }
}
