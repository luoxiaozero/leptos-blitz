use super::node::{Node, NodeId};

pub struct Text(Node);

impl From<NodeId> for Text {
    fn from(value: NodeId) -> Self {
        Self(Node::from(value))
    }
}
