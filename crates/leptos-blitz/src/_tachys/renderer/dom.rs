use super::{CastFrom};
use crate::_tachys::view::Mountable;
use crate::web_document::{self, WebDocument};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

pub type Node = web_document::Node;
pub type Text = web_document::Text;
pub type Element = web_document::Element;
pub type Placeholder = web_document::Comment;
// pub type Event = wasm_bindgen::JsValue;
// pub type ClassList = web_document::DomTokenList;
// pub type CssStyleDeclaration = web_document::CssStyleDeclaration;
// pub type TemplateElement = web_document::HtmlTemplateElement;

impl Dom {
    pub fn intern(text: &str) -> &str {
        text
    }

    pub fn create_element(tag: &str, namespace: Option<&str>) -> Element {
        WebDocument::create_element_ns(namespace, tag)
    }

    pub fn create_text_node(text: &str) -> Text {
        WebDocument::create_text_node(text)
    }

    pub fn create_placeholder() -> Placeholder {
        WebDocument::create_comment()
    }

    pub fn set_text(node: &Text, text: &str) {
        node.set_text_content(text);
    }

    pub fn set_attribute(node: &Element, name: &str, value: &str) {
        node.set_attribute(name, value);
    }

    pub fn remove_attribute(node: &Element, name: &str) {
        node.remove_attribute(name);
    }

    pub fn insert_node(parent: &Element, new_child: &Node, anchor: Option<&Node>) {
        parent.insert_before(new_child, anchor);
    }

    pub fn remove_node(parent: &Element, child: &Node) -> Option<Node> {
        parent.remove_child(child)
    }

    pub fn remove(node: &Element) {
        node.remove();
    }

    pub fn get_parent(node: &Node) -> Option<Node> {
        node.parent_node()
    }

    pub fn first_child(node: &Node) -> Option<Node> {
        node.first_child()
    }

    pub fn next_sibling(node: &Node) -> Option<Node> {
        node.next_sibling()
    }

    pub fn log_node(node: &Node) {
        let doc = WebDocument::document();
        let node = doc.get_node(node.node_id());
        println!("{:#?}", node);
    }

    pub fn clear_children(parent: &Element) {
        let doc = WebDocument::document_mut();
        let parent = doc.get_node_mut(parent.node_id()).unwrap();
        let children = parent.children.clone();
        for child in children.into_iter() {
            doc.remove_node(child);
        }
    }
}

impl Mountable for Node {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(&mut self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        let parent = Dom::get_parent(self).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self));
            return true;
        }
        false
    }
}

impl Mountable for Text {
    fn unmount(&mut self) {
        self.remove();
    }

    fn mount(&mut self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        let parent = Dom::get_parent(self.as_ref()).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self));
            return true;
        }
        false
    }
}

impl Mountable for web_document::Comment {
    fn unmount(&mut self) {
        self.remove();
    }

    fn mount(&mut self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, &self, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        let parent = Dom::get_parent(&self).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self));
            return true;
        }
        false
    }
}

impl Mountable for Element {
    fn unmount(&mut self) {
        self.remove();
    }

    fn mount(&mut self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        let parent = Dom::get_parent(&self).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self));
            return true;
        }
        false
    }
}

impl CastFrom<Node> for Element {
    fn cast_from(node: Node) -> Option<Element> {
        let doc = WebDocument::document();
        let node_id = node.node_id();
        let node = doc.get_node(node_id).unwrap();
        node.is_element().then_some(Element::from(node_id))
    }
}
