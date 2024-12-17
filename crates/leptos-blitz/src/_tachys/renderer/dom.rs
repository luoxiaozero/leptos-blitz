#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

pub use node::*;

mod node {
    type NodeId = usize;

    #[derive(Debug, Clone)]
    pub struct Node(NodeId);

    #[derive(Debug, Clone)]
    pub struct Element(Node);

    impl From<usize> for Element {
        fn from(value: usize) -> Self {
            Self(Node(value))
        }
    }
}
