use crate::{documents::LeptosDocument, dom::qual_name};
use blitz_dom::{node::Attribute, Atom, ElementNodeData, NodeData, QualName};
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
        let display = Attribute {
            name: QualName {
                prefix: None,
                ns: Atom::from("display"),
                local: Atom::from("display"),
            },
            value: "none".to_string(),
        };

        let data = ElementNodeData::new(qual_name("div", None), vec![display]);

        // NodeData::Comment
        let id = LeptosDocument::document_mut().create_node(NodeData::Element(data));

        Element(Node(id))
    }

    fn set_text(node: &Self::Text, text: &str) {
        let doc = LeptosDocument::document_mut();
        let Some(node) = doc.get_node_mut(node.node_id()) else {
            return;
        };

        if let Some(text_data) = node.text_data_mut() {
            text_data.content = text.to_string();
        }
    }

    fn set_attribute(node: &Self::Element, name: &str, value: &str) {
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

    fn remove_attribute(node: &Self::Element, name: &str) {
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

    fn insert_node(parent: &Self::Element, new_child: &Self::Node, marker: Option<&Self::Node>) {
        let doc = LeptosDocument::document_mut();
        if let Some(marker) = marker {
            doc.insert_before(new_child.0, &[marker.0]);
        } else {
            let parent_id = parent.node_id();
            let child_idx = {
                let parent = doc.get_node_mut(parent_id).unwrap();
                parent.children.push(new_child.0);
                parent.children.len() - 1
            };

            let node = doc.get_node_mut(new_child.node_id()).unwrap();
            node.child_idx = child_idx;
            node.parent = Some(parent_id);
        }
    }

    fn remove_node(_parent: &Self::Element, child: &Self::Node) -> Option<Self::Node> {
        let doc = LeptosDocument::document_mut();
        let node = doc.remove_node(child.node_id());

        // (Node, blitz::Node) WeakMap
        node.map(|node| Node(node.id))
    }

    fn clear_children(parent: &Self::Element) {
        let doc = LeptosDocument::document_mut();

        let parent = doc.get_node_mut(parent.node_id()).unwrap();
        let children = parent.children.clone();
        for child in children.into_iter() {
            doc.remove_node(child.clone());
        }
    }

    fn remove(node: &Self::Node) {
        let doc = LeptosDocument::document_mut();
        doc.remove_node(node.node_id());
    }

    fn get_parent(node: &Self::Node) -> Option<Self::Node> {
        let doc = LeptosDocument::document();
        let node = doc.get_node(node.node_id())?;
        node.parent.map(|parent| Node(parent))
    }

    fn first_child(node: &Self::Node) -> Option<Self::Node> {
        let doc = LeptosDocument::document();
        let node = doc.get_node(node.node_id())?;
        node.children.first().map(|child| Node(child.clone()))
    }

    fn next_sibling(node: &Self::Node) -> Option<Self::Node> {
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

    fn log_node(node: &Self::Node) {
        let doc = LeptosDocument::document();
        let node = doc.get_node(node.node_id());
        println!("{:#?}", node);
    }
}

impl Mountable<LeptosDom> for Node {
    fn unmount(&mut self) {
        LeptosDom::remove(self);
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        LeptosDom::insert_node(parent, self, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        let parent = LeptosDom::get_parent(self).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self));
            return true;
        }
        false
    }
}

type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node(pub NodeId);

impl Node {
    pub fn node_id(&self) -> NodeId {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Element(pub Node);

impl Element {
    pub fn node_id(&self) -> NodeId {
        self.0.node_id()
    }
}

impl Mountable<LeptosDom> for Element {
    fn unmount(&mut self) {
        LeptosDom::remove(self.as_ref());
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        LeptosDom::insert_node(parent, self.as_ref(), marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        let parent = LeptosDom::get_parent(self.as_ref()).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self.as_ref()));
            return true;
        }
        false
    }
}

impl CastFrom<Node> for Element {
    fn cast_from(source: Node) -> Option<Self> {
        let doc = LeptosDocument::document_mut();
        let Some(node) = doc.get_node(source.node_id()) else {
            return None;
        };
        if node.is_element() {
            Some(Self(source))
        } else {
            None
        }
    }
}

impl AsRef<Node> for Element {
    fn as_ref(&self) -> &Node {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Text(pub Node);

impl Text {
    pub fn node_id(&self) -> NodeId {
        self.0.node_id()
    }
}

impl Mountable<LeptosDom> for Text {
    fn unmount(&mut self) {
        LeptosDom::remove(self.as_ref());
    }

    fn mount(
        &mut self,
        parent: &<LeptosDom as Renderer>::Element,
        marker: Option<&<LeptosDom as Renderer>::Node>,
    ) {
        LeptosDom::insert_node(parent, self.as_ref(), marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<LeptosDom>) -> bool {
        println!("33333");
        let parent = LeptosDom::get_parent(self.as_ref()).and_then(Element::cast_from);
        if let Some(parent) = parent {
            child.mount(&parent, Some(self.as_ref()));
            return true;
        }
        false
    }
}

impl CastFrom<Node> for Text {
    fn cast_from(source: Node) -> Option<Self> {
        let doc = LeptosDocument::document_mut();
        let Some(node) = doc.get_node(source.node_id()) else {
            return None;
        };
        if node.is_text_node() {
            Some(Self(source))
        } else {
            None
        }
    }
}

impl AsRef<Node> for Text {
    fn as_ref(&self) -> &Node {
        &self.0
    }
}
