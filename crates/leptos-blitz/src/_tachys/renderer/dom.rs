use super::CastFrom;
use crate::_tachys::view::Mountable;
use blitz_web_api::dom::{self, window, BlitzDocument};

fn document() -> dom::Document {
    window().document().clone()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dom;

pub type Node = dom::Node;
pub type Text = dom::Text;
pub type Element = dom::Element;
pub type Placeholder = dom::Comment;
// pub type Event = wasm_bindgen::JsValue;
// pub type ClassList = web_document::DomTokenList;
// pub type CssStyleDeclaration = web_document::CssStyleDeclaration;
// pub type TemplateElement = web_document::HtmlTemplateElement;

impl Dom {
    pub fn intern(text: &str) -> &str {
        text
    }

    pub fn create_element(tag: &str, namespace: Option<&str>) -> Element {
        document().create_element_ns(namespace, tag)
    }

    pub fn create_text_node(text: &str) -> Text {
        document().create_text_node(text)
    }

    pub fn create_placeholder() -> Placeholder {
        document().create_comment("")
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

    pub fn remove_self(node: &Node) {
        let doc = BlitzDocument::document_mut();
        doc.remove_node(node.node_id());
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
        let doc = BlitzDocument::document();
        let node = doc.get_node(node.node_id());
        println!("{:#?}", node);
        if let Some(node) = node {
            println!("{:#?}", node.outer_html());
        }
    }

    pub fn clear_children(parent: &Element) {
        let doc = BlitzDocument::document_mut();
        let parent = doc.get_node_mut(parent.node_id()).unwrap();
        let children = parent.children.clone();
        for child in children.into_iter() {
            doc.remove_node(child);
        }
    }

    // pub fn clone_template(tpl: &TemplateElement) -> Element {
    //     todo!()
    // tpl.content()
    //     .clone_node_with_deep(true)
    //     .unwrap()
    //     .unchecked_into()
    // }

    pub fn create_element_from_html(html: &str) -> Element {
        use html5ever::parse_document;
        use markup5ever::tendril::TendrilSink;
        use markup5ever_rcdom::{Handle, NodeData, RcDom};

        let tpl = Self::create_element("div", None);

        let dom: RcDom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap();

        fn traverse_dom(parent: &Element, handle: &Handle) {
            for child in handle.children.borrow().iter() {
                match &child.data {
                    NodeData::Text { contents } => {
                        let node = document().create_text_node(&contents.borrow());
                        parent.insert_before(&node, None);
                    }
                    NodeData::Comment { contents: _ } => {
                        let node = document().create_comment("");
                        parent.insert_before(&node, None);
                    }
                    NodeData::Element {
                        name,
                        attrs,
                        template_contents: _,
                        mathml_annotation_xml_integration_point: _,
                    } => {
                        let name: &str = &name.local;
                        let node = if ["html", "head", "body"].contains(&name) {
                            parent.clone()
                        } else {
                            let node = document().create_element_ns(None, name);
                            for attr in attrs.borrow().iter() {
                                node.set_attribute(&attr.name.local, &attr.value);
                            }
                            parent.insert_before(&node, None);
                            node
                        };

                        traverse_dom(&node, child);
                    }
                    _ => {}
                }
            }
        }
        traverse_dom(&tpl, &dom.document);
        // Self::log_node(&tpl);
        tpl
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
        Dom::remove_self(self);
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

impl CastFrom<Node> for Text {
    fn cast_from(node: Node) -> Option<Self> {
        node.try_into().ok()
    }
}

impl Mountable for dom::Comment {
    fn unmount(&mut self) {
        Dom::remove_self(self);
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

impl CastFrom<Node> for dom::Comment {
    fn cast_from(node: Node) -> Option<Self> {
        node.try_into().ok()
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
        node.try_into().ok()
    }
}
