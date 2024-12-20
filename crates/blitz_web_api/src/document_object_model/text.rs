use super::{
    node::{Node, NodeId},
    BlitzDocument, DomError,
};
use std::ops::Deref;

pub struct Text(Node);

impl TryFrom<Node> for Text {
    type Error = DomError;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        let doc = BlitzDocument::document();
        let node_id = value.node_id();
        let node = doc.get_node(node_id).unwrap();
        if node.is_text_node() {
            Ok(Self::from(node_id))
        } else {
            Err(DomError::Type("Node", "Text"))
        }
    }
}

impl Deref for Text {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NodeId> for Text {
    fn from(value: NodeId) -> Self {
        Self(Node::from(value))
    }
}
