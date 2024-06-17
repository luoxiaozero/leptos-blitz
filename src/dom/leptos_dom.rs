use leptos::tachys::{
    renderer::{CastFrom, Renderer},
    view::Mountable,
};
// use crate::documents::LeptosDocument;

#[derive(Debug)]
pub struct LeptosDom;

impl Renderer for LeptosDom {
    type Node = Node;
    type Element = Element;
    type Text = Text;
    type Placeholder = Element;

    fn intern(text: &str) -> &str {
        todo!()
    }

    fn create_text_node(text: &str) -> Self::Text {
        todo!()
    }

    fn create_placeholder() -> Self::Placeholder {
        todo!()
    }

    fn set_text(node: &Self::Text, text: &str) {
        todo!()
    }

    fn set_attribute(node: &Self::Element, name: &str, value: &str) {
        todo!()
    }

    fn remove_attribute(node: &Self::Element, name: &str) {
        todo!()
    }

    fn insert_node(parent: &Self::Element, new_child: &Self::Node, marker: Option<&Self::Node>) {
        todo!()
    }

    fn remove_node(parent: &Self::Element, child: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn clear_children(parent: &Self::Element) {
        todo!()
    }

    fn remove(node: &Self::Node) {
        todo!()
    }

    fn get_parent(node: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn first_child(node: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn next_sibling(node: &Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn log_node(node: &Self::Node) {
        todo!()
    }
}

impl Mountable<LeptosDom> for Node {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct Node(pub blitz_dom::Node);

impl Clone for Node {
    fn clone(&self) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Element(pub Node);

impl Element {
    pub unsafe fn convert_from_node<'a>(node: &'a blitz_dom::Node) -> &'a Element {
        let node_ptr: *const blitz_dom::Node = node;
        let element_ptr: *const Element = node_ptr.cast();
        &*element_ptr
    }
}

impl Mountable<LeptosDom> for Element {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        todo!()
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        todo!()
    }
}

impl CastFrom<Node> for Element {
    fn cast_from(source: Node) -> Option<Self> {
        todo!()
    }
}

impl AsRef<Node> for Element {
    fn as_ref(&self) -> &Node {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Text(pub blitz_dom::TextNodeData);

impl Mountable<LeptosDom> for Text {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        todo!()
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        todo!()
    }
}

impl CastFrom<Node> for Text {
    fn cast_from(source: Node) -> Option<Self> {
        todo!()
    }
}

impl AsRef<Node> for Text {
    fn as_ref(&self) -> &Node {
        todo!()
    }
}
