use crate::{leptos_document::qual_name, LeptosDocument};
use blitz_dom::{ElementNodeData, NodeData, node::Attribute};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

pub use node::*;

pub type Placeholder = node::Comment;

mod node {
    use std::ops::Deref;

    type NodeId = usize;

    #[derive(Debug, Clone)]
    pub struct Node(NodeId);

    impl Node {
        pub(super) fn node_id(&self) -> usize {
            self.0.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct Element(Node);

    impl Deref for Element {
        type Target = Node;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<usize> for Element {
        fn from(value: usize) -> Self {
            Self(Node(value))
        }
    }

    #[derive(Debug, Clone)]
    pub struct Text(Node);

    impl Deref for Text {
        type Target = Node;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<usize> for Text {
        fn from(value: usize) -> Self {
            Self(Node(value))
        }
    }

    #[derive(Debug, Clone)]
    pub struct Comment(Node);

    impl From<usize> for Comment {
        fn from(value: usize) -> Self {
            Self(Node(value))
        }
    }
}

impl Dom {
    pub fn intern(text: &str) -> &str {
        text
    }

    pub fn create_element(tag: &str, namespace: Option<&str>) -> Element {
        let data = ElementNodeData::new(qual_name(tag, namespace), vec![]);
        let id = LeptosDocument::document_mut().create_node(NodeData::Element(data));
        Element::from(id)
    }

    fn create_text_node(text: &str) -> Text {
        let doc = LeptosDocument::document_mut();
        let id = doc.create_text_node(text);
        Text::from(id)
    }

    fn create_placeholder() -> Placeholder {
        let id = LeptosDocument::document_mut().create_node(NodeData::Comment);
        Comment::from(id)
    }

    fn set_text(node: &Text, text: &str) {
        let doc = LeptosDocument::document_mut();
        let Some(node) = doc.get_node_mut(node.node_id()) else {
            return;
        };
        if let Some(text_data) = node.text_data_mut() {
            text_data.content = text.to_string();
        }
    }

    fn set_attribute(node: &Element, name: &str, value: &str) {
        let doc = LeptosDocument::document_mut();
        let node = doc.get_node_mut(node.node_id()).unwrap();

        if let NodeData::Element(ref mut element) = node.raw_dom_data {
            let existing_attr = element
                .attrs
                .iter_mut()
                .find(|attr| attr.name.local == *name);
            if let Some(existing_attr) = existing_attr {
                existing_attr.value = value.to_string();
            } else {
                element.attrs.push(Attribute {
                    name: qual_name(name, None),
                    value: value.to_string(),
                });
            }
            if name == "style" {
                let doc = LeptosDocument::document();
                element.flush_style_attribute(doc.guard());
            }
        }
    }

    fn remove_attribute(node: &Element, name: &str) {
        let doc = LeptosDocument::document_mut();
        let node = doc.get_node_mut(node.node_id()).unwrap();

        if let NodeData::Element(ref mut element) = node.raw_dom_data {
            if let Some(position) = element
                .attrs
                .iter_mut()
                .position(|attr| attr.name.local == *name)
            {
                element.attrs.remove(position);
            }
        }
    }

    fn insert_node(parent: &Element, new_child: &Node, marker: Option<&Node>) {
        let doc = LeptosDocument::document_mut();
        if let Some(marker) = marker {
            doc.insert_before(new_child.node_id(), &[marker.node_id()]);
        } else {
            let parent_id = parent.node_id();
            let child_idx = {
                let parent = doc.get_node_mut(parent_id).unwrap();
                parent.children.push(new_child.node_id());
                parent.children.len() - 1
            };

            let node = doc.get_node_mut(new_child.node_id()).unwrap();
            node.child_idx = child_idx;
            node.parent = Some(parent_id);
        }
    }

    fn remove_node(_parent: &Element, child: &Node) -> Option<Node> {
        let doc = LeptosDocument::document_mut();
        let node = doc.remove_node(child.node_id());

        // (Node, blitz::Node) WeakMap
        node.map(|node| Node(node.id))
    }

    fn clear_children(parent: &Element) {
        let doc = LeptosDocument::document_mut();

        let parent = doc.get_node_mut(parent.node_id()).unwrap();
        let children = parent.children.clone();
        for child in children.into_iter() {
            doc.remove_node(child.clone());
        }
    }

    fn remove(node: &Node) {
        let doc = LeptosDocument::document_mut();
        doc.remove_node(node.node_id());
    }

    fn get_parent(node: &Node) -> Option<Node> {
        let doc = LeptosDocument::document();
        let node = doc.get_node(node.node_id())?;
        node.parent.map(|parent| Node(parent))
    }

    fn first_child(node: &Node) -> Option<Node> {
        let doc = LeptosDocument::document();
        let node = doc.get_node(node.node_id())?;
        node.children.first().map(|child| Node(child.clone()))
    }

    fn next_sibling(node: &Node) -> Option<Node> {
        let parent = LeptosDom::get_parent(node).expect("Parent Node");
        let doc = LeptosDocument::document();
        let parent = doc.get_node(parent.node_id()).expect("Parent Node");

        let index = parent
            .children
            .iter()
            .find(|child| child == &&node.node_id())
            .unwrap();

        if index + 1 < parent.children.len() {
            Some(Node(index + 1))
        } else {
            None
        }
    }

    fn log_node(node: &Node) {
        let doc = LeptosDocument::document();
        let node = doc.get_node(node.node_id());
        println!("{:#?}", node);
    }
}
