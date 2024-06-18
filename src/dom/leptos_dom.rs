use crate::LeptosDocument;
use leptos::tachys::{
    renderer::{CastFrom, Renderer},
    view::Mountable,
};

#[derive(Debug)]
pub struct LeptosDom;

impl Renderer for LeptosDom {
    type Node = Node;
    type Element = Element;
    type Text = Text;
    type Placeholder = Element;

    fn intern(text: &str) -> &str {
        text
    }

    fn create_text_node(text: &str) -> Self::Text {
        let doc = LeptosDocument::document_mut();
        let id = doc.create_text_node(text);
        Text(Node(id))
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
        let doc = LeptosDocument::document_mut();
        if let Some(marker) = marker {
            doc.insert_before(new_child.0, &[marker.0]);
        } else {
            let parent_id = parent.0.0;
            let parent = doc.get_node_mut(parent_id).unwrap();
            parent.children.push(new_child.0);
        }
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
        LeptosDom::insert_node(parent, self, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Node(pub usize);

#[derive(Debug, Clone)]
pub struct Element(pub Node);

impl Mountable<LeptosDom> for Element {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        LeptosDom::insert_node(parent, self.as_ref(), marker);
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
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Text(pub Node);

impl Mountable<LeptosDom> for Text {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        LeptosDom::insert_node(parent, self.as_ref(), marker);
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
        &self.0
    }
}
