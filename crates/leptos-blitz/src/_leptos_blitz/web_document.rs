use super::leptos_document::qual_name;
use blitz_dom::{
    local_name, namespace_url,
    node::{Attribute, NodeSpecificData},
    ns, Document, ElementNodeData, NodeData, QualName, RestyleHint,
};
use std::cell::RefCell;
use std::ops::Deref;

thread_local! {
    static DOCUMENT: RefCell<Option<Document>> = RefCell::new(None);
}

pub struct WebDocument;

impl WebDocument {
    pub fn set_document(doc: Document) {
        DOCUMENT.with(|document| {
            *document.borrow_mut() = Some(doc);
        });
    }

    pub fn document() -> &'static Document {
        DOCUMENT.with(|doc| {
            let borrowed_doc = doc.borrow();
            if let Some(ref document) = *borrowed_doc {
                unsafe { std::mem::transmute::<&Document, &'static Document>(document) }
            } else {
                panic!("Document is None");
            }
        })
    }

    pub fn document_mut() -> &'static mut Document {
        DOCUMENT.with(|doc| {
            let mut borrowed_doc = doc.borrow_mut();
            if let Some(ref mut document) = *borrowed_doc {
                unsafe { std::mem::transmute::<&mut Document, &'static mut Document>(document) }
            } else {
                panic!("Document is None");
            }
        })
    }

    pub fn document_take() -> Document {
        DOCUMENT.with(|doc| {
            let mut borrowed_doc = doc.borrow_mut();
            if let Some(document) = borrowed_doc.take() {
                document
            } else {
                panic!("Document is None");
            }
        })
    }
}

impl WebDocument {
    pub fn create_comment() -> Comment {
        let id = Self::document_mut().create_node(NodeData::Comment);
        Comment::from(id)
    }

    pub fn create_element_ns(namespace_url: Option<&str>, qualified_name: &str) -> Element {
        let data = ElementNodeData::new(qual_name(qualified_name, namespace_url), vec![]);
        let id = Self::document_mut().create_node(NodeData::Element(data));
        Element::from(id)
    }

    pub fn create_text_node(data: &str) -> Text {
        let doc = Self::document_mut();
        let id = doc.create_text_node(data);
        Text::from(id)
    }
}

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node(NodeId);

impl Node {
    pub fn node_id(&self) -> NodeId {
        self.0.clone()
    }

    pub fn parent_node(&self) -> Option<Node> {
        let doc = WebDocument::document();
        let node = doc.get_node(self.node_id())?;
        node.parent.map(|parent| Node(parent))
    }

    pub fn first_child(&self) -> Option<Node> {
        let doc = WebDocument::document();
        let node = doc.get_node(self.node_id())?;
        node.children.first().map(|child| Node(child.clone()))
    }

    pub fn next_sibling(&self) -> Option<Node> {
        let doc = WebDocument::document();
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

    pub fn insert_before(&self, new_node: &Node, reference_node: Option<&Node>) {
        let doc = WebDocument::document_mut();
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

    pub fn remove_child(&self, child: &Node) -> Option<Node> {
        let doc = WebDocument::document_mut();
        doc.remove_node(child.node_id());
        Some(child.clone())
    }

    pub fn set_text_content(&self, value: &str) {
        let doc = WebDocument::document_mut();
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

    fn maybe_update_style_node(doc: &mut Document, node_id: Option<NodeId>) {
        if let Some(node_id) = node_id {
            if Self::is_style_node(doc, node_id) {
                doc.upsert_stylesheet_for_node(node_id);
            }
        }
    }

    fn is_style_node(doc: &Document, node_id: NodeId) -> bool {
        doc.get_node(node_id)
            .unwrap()
            .raw_dom_data
            .is_element_with_tag_name(&local_name!("style"))
    }
}

#[derive(Debug, Clone)]
pub struct Element(Node);

impl Element {
    pub fn set_attribute(&self, name: &str, value: &str) {
        let doc = WebDocument::document_mut();

        doc.snapshot_node(self.node_id());
        // let node = doc.get_node_mut(self.node_id()).unwrap();
        let node = &mut doc.nodes[self.node_id()];

        let stylo_element_data = &mut *node.stylo_element_data.borrow_mut();
        if let Some(data) = stylo_element_data {
            data.hint |= RestyleHint::restyle_subtree();
        }

        if let NodeData::Element(ref mut element) = node.raw_dom_data {
            if element.name.local == local_name!("input") && name == "checked" {
                set_input_checked_state(element, value);
            }
            // FIXME: support other non-text attributes
            else {
                let val = value;
                if name == "value" {
                    // Update text input value
                    if let Some(input_data) = element.text_input_data_mut() {
                        input_data.set_text(&mut doc.font_ctx, &mut doc.layout_ctx, val);
                    }
                }

                // FIXME check namespace
                let existing_attr = element
                    .attrs
                    .iter_mut()
                    .find(|attr| attr.name.local == *name);

                if let Some(existing_attr) = existing_attr {
                    existing_attr.value.clear();
                    existing_attr.value.push_str(val);
                } else {
                    // TODO
                    let ns = None;
                    // we have overloaded the style namespace to accumulate style attributes without a `style` block
                    if ns == Some("style") {
                        // todo: need to accumulate style attributes into a single style
                        //
                        // element.
                    } else {
                        element.attrs.push(Attribute {
                            name: qual_name(name, ns),
                            value: val.to_string(),
                        });

                        // TODO
                        if name == "style" {
                            let doc = WebDocument::document();
                            element.flush_style_attribute(doc.guard());
                        }
                    }
                }
            }
        }
    }

    pub fn remove_attribute(&self, attr_name: &str) {
        let name = attr_name;
        let doc = WebDocument::document_mut();

        doc.snapshot_node(self.node_id());
        // let node = doc.get_node_mut(self.node_id()).unwrap();
        let node = &mut doc.nodes[self.node_id()];

        let stylo_element_data = &mut *node.stylo_element_data.borrow_mut();
        if let Some(data) = stylo_element_data {
            data.hint |= RestyleHint::restyle_subtree();
        }

        if let NodeData::Element(ref mut element) = node.raw_dom_data {
            if element.name.local == local_name!("input") && name == "checked" {
                return;
            }
            // FIXME: support other non-text attributes
            else {
                // Update text input value
                if name == "value" {
                    if let Some(input_data) = element.text_input_data_mut() {
                        input_data.set_text(&mut doc.font_ctx, &mut doc.layout_ctx, "");
                    }
                }

                // FIXME: check namespace
                element.attrs.retain(|attr| attr.name.local != *name);
            }
        }
    }

    pub fn remove(&self) {
        let doc = WebDocument::document_mut();
        doc.remove_node(self.node_id());
    }
}

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

impl Deref for Comment {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for Comment {
    fn from(value: usize) -> Self {
        Self(Node(value))
    }
}

/// Set 'checked' state on an input based on given attributevalue
fn set_input_checked_state(element: &mut ElementNodeData, value: &str) {
    let Ok(checked) = value.parse() else {
        return;
    };

    match element.node_specific_data {
        NodeSpecificData::CheckboxInput(ref mut checked_mut) => *checked_mut = checked,
        // If we have just constructed the element, set the node attribute,
        // and NodeSpecificData will be created from that later
        // this simulates the checked attribute being set in html,
        // and the element's checked property being set from that
        NodeSpecificData::None => element.attrs.push(Attribute {
            name: QualName {
                prefix: None,
                ns: ns!(html),
                local: local_name!("checked"),
            },
            value: checked.to_string(),
        }),
        _ => {}
    }
}
