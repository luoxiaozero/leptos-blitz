use super::blitz_document::BlitzDocument;
use blitz_dom::{local_name, NodeData};

pub(super) type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node(NodeId);

impl Node {
    pub fn node_id(&self) -> NodeId {
        self.0.clone()
    }

    fn maybe_update_style_node(doc: &mut blitz_dom::Document, node_id: Option<NodeId>) {
        if let Some(node_id) = node_id {
            if Self::is_style_node(doc, node_id) {
                doc.upsert_stylesheet_for_node(node_id);
            }
        }
    }

    fn is_style_node(doc: &blitz_dom::Document, node_id: NodeId) -> bool {
        doc.get_node(node_id)
            .unwrap()
            .raw_dom_data
            .is_element_with_tag_name(&local_name!("style"))
    }
}

impl Node {
    #[doc = "Getter for the `parentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)"]
    pub fn parent_node(&self) -> Option<Node> {
        let node = BlitzDocument::document().get_node(self.node_id())?;
        node.parent.map(|parent| Node(parent))
    }

    #[doc = "Getter for the `firstChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)"]
    pub fn first_child(&self) -> Option<Node> {
        let node = BlitzDocument::document().get_node(self.node_id())?;
        node.children.first().map(|child| Node(child.clone()))
    }

    #[doc = "Getter for the `nextSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)"]
    pub fn next_sibling(&self) -> Option<Node> {
        let doc = BlitzDocument::document();
        let node = doc.get_node(self.node_id()).unwrap();
        let parent = doc.get_node(node.parent.unwrap()).unwrap();

        let index = parent
            .children
            .iter()
            .find(|child| child == &&self.node_id())
            .unwrap();

        if index + 1 < parent.children.len() {
            Some(Node(index + 1))
        } else {
            None
        }
    }

    #[doc = "The `insertBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)"]
    pub fn insert_before(&self, new_node: &Node, reference_node: Option<&Node>) {
        let doc = BlitzDocument::document_mut();
        if let Some(reference_node) = reference_node {
            // TODO Verify that reference_node's parent is self
            doc.insert_before(reference_node.node_id(), &[new_node.node_id()]);
        } else {
            let parent_id = self.node_id();
            {
                let parent = doc.get_node_mut(parent_id).unwrap();
                parent.children.push(new_node.node_id());
            }
            let node = doc.get_node_mut(new_node.node_id()).unwrap();
            node.parent = Some(parent_id);
        }
    }

    #[doc = "The `removeChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)"]
    pub fn remove_child(&self, child: &Node) -> Option<Node> {
        BlitzDocument::document_mut().remove_node(child.node_id());
        Some(child.clone())
    }

    #[doc = "Setter for the `textContent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)"]
    pub fn set_text_content(&self, value: &str) {
        let doc = BlitzDocument::document_mut();
        let node = doc.get_node_mut(self.node_id()).unwrap();

        let text = match node.raw_dom_data {
            NodeData::Text(ref mut text) => text,
            // todo: otherwise this is basically element.textContent which is a bit different - need to parse as html
            _ => return,
        };

        let changed = text.content != value;
        if changed {
            text.content.clear();
            text.content.push_str(value);
            let parent = node.parent;
            Self::maybe_update_style_node(doc, parent);
        }
    }
}

impl From<NodeId> for Node {
    fn from(value: NodeId) -> Self {
        Self(value)
    }
}
