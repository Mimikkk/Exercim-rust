

pub mod graph_items{
    use std::fmt::Debug;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Eq)]
    pub mod node {
    
    }
}

pub mod graph {
    use std::collections::HashMap;

    pub struct Graph{
        pub edges: Vec<Node>,
        pub nodes: Vec<nodes>,
        pub attrs: HashMap<String, String>,
    }

    pub struct Edge{
        start: String,
        end: String,
    }
    pub struct Node{
        name: String,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {edges: Vec::new(), nodes: Vec::new(), attrs: HashMap::new()}

        }
    }
}

// NODE, EDGE, ATTR = range(3)
//
//
// class Node:
// def __init__(self, name, attrs):
// self.name = name
// self.attrs = attrs
//
// def __eq__(self, other):
// return self.name == other.name and self.attrs == other.attrs
//
//
// class Edge:
// def __init__(self, src, dst, attrs):
// self.src = src
// self.dst = dst
// self.attrs = attrs
//
// def __eq__(self, other):
// return (self.src == other.src and
// self.dst == other.dst and
// self.attrs == other.attrs)
//
//
// class Graph:
// def __init__(self, data=None):
// pass
