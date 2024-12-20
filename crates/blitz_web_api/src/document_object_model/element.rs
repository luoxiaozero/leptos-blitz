use super::{
    blitz_document, blitz_document_mut,
    document::qual_name,
    node::{Node, NodeId},
};
use blitz_dom::{
    local_name,
    node::{Attribute, NodeSpecificData},
    ns, ElementNodeData, NodeData, QualName, RestyleHint,
};
use std::ops::Deref;

pub struct Element(Node);

impl Element {
    pub fn set_attribute(&self, name: &str, value: &str) {
        let doc = blitz_document_mut();

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
                            let doc = blitz_document();
                            element.flush_style_attribute(doc.guard());
                        }
                    }
                }
            }
        }
    }

    pub fn remove_attribute(&self, attr_name: &str) {
        let name = attr_name;
        let doc = blitz_document_mut();

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
        let doc = blitz_document_mut();
        doc.remove_node(self.node_id());
    }
}

impl Deref for Element {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NodeId> for Element {
    fn from(value: NodeId) -> Self {
        Self(Node::from(value))
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
