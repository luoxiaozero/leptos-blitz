use super::{
    blitz_document_mut,
    comment::Comment,
    element::Element,
    node::{Node, NodeId},
    text::Text,
};
use blitz_dom::{ns, Atom, ElementNodeData, NodeData, QualName};

pub(super) fn qual_name(local_name: &str, namespace: Option<&str>) -> QualName {
    QualName {
        prefix: None,
        ns: namespace.map(Atom::from).unwrap_or(ns!(html)),
        local: Atom::from(local_name),
    }
}

#[derive(Debug, Clone)]
pub struct Document(Node);

impl Document {
    #[doc = "The `createComment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment)"]
    pub fn create_comment(&self, _data: &str) -> Comment {
        let id = blitz_document_mut().create_node(NodeData::Comment);
        Comment::from(id)
    }

    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    pub fn create_element_ns(&self, namespace_url: Option<&str>, qualified_name: &str) -> Element {
        let data = ElementNodeData::new(qual_name(qualified_name, namespace_url), vec![]);
        let id = blitz_document_mut().create_node(NodeData::Element(data));
        Element::from(id)
    }

    #[doc = "The `createTextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)"]
    pub fn create_text_node(&self, data: &str) -> Text {
        let id = blitz_document_mut().create_text_node(data);
        Text::from(id)
    }
}

impl From<NodeId> for Document {
    fn from(value: NodeId) -> Self {
        Self(Node::from(value))
    }
}
