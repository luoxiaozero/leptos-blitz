use super::node::{Node, NodeId};
use std::ops::Deref;

pub struct Comment(Node);

impl Deref for Comment {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NodeId> for Comment {
    fn from(value: NodeId) -> Self {
        Self(Node::from(value))
    }
}
