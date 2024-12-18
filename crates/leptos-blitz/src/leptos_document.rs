use crate::_leptos::{
    mount_to,
    prelude::{Mountable, Owner},
    IntoView,
};
use crate::web_document::WebDocument;
use blitz_dom::{
    namespace_url, net::Resource, ns, Atom, ColorScheme, Document, DocumentLike, ElementNodeData,
    NodeData, QualName, Viewport, DEFAULT_CSS,
};
use blitz_traits::net::NetProvider;
use futures_util::FutureExt;
use std::sync::Arc;
use tokio::task::LocalSet;

pub(crate) fn qual_name(local_name: &str, namespace: Option<&str>) -> QualName {
    QualName {
        prefix: None,
        ns: namespace.map(Atom::from).unwrap_or(ns!(html)),
        local: Atom::from(local_name),
    }
}

pub struct LeptosDocument {
    #[allow(dead_code)]
    owner: Owner,
    mountable: Box<dyn Mountable>,
    local_set: LocalSet,
    // inner: Document,
}

impl AsRef<Document> for LeptosDocument {
    fn as_ref(&self) -> &Document {
        WebDocument::document()
    }
}
impl AsMut<Document> for LeptosDocument {
    fn as_mut(&mut self) -> &mut Document {
        WebDocument::document_mut()
    }
}

impl From<LeptosDocument> for Document {
    fn from(_doc: LeptosDocument) -> Document {
        WebDocument::document_take()
    }
}

impl DocumentLike for LeptosDocument {
    fn poll(&mut self, mut cx: std::task::Context) -> bool {
        let _ = self.local_set.poll_unpin(&mut cx);
        true
    }

    fn handle_event(&mut self, event: blitz_dom::events::RendererEvent) {
        // Collect the nodes into a chain by traversing upwards
        // This is important so the "capture" phase can be implemented
        let mut next_node_id = Some(event.target);
        // let mut chain = Vec::with_capacity(16);

        // if it's a capturing event, we want to fill in the chain with the parent nodes
        // until we reach the root - that way we can call the listeners in the correct order
        // otherwise, we just want to call the listeners on the target
        //
        // todo: this is harcoded for "click" events - eventually we actually need to handle proper propagation
        // if event.name == "click" {
        while let Some(node_id) = next_node_id {
            let node = &self.inner().tree()[node_id];

            if let Some(element) = node.element_data() {
                // let dioxus_id = DioxusDocument::dioxus_id(element);
                // chain.push(DxNodeIds { node_id, dioxus_id })
                // chain.push(parent);
            }

            next_node_id = node.parent;
        }
    }
}

impl LeptosDocument {
    pub fn new<F, N>(
        rt: &tokio::runtime::Runtime,
        f: F,
        net_provider: Option<Arc<dyn NetProvider<Data = Resource>>>,
    ) -> Self
    where
        F: FnOnce() -> N + 'static,
        N: IntoView + 'static,
    {
        let viewport = Viewport::new(0, 0, 1.0, ColorScheme::Light);
        let mut doc = Document::new(viewport);

        // Set net provider
        if let Some(net_provider) = net_provider {
            doc.set_net_provider(net_provider);
        }

        // Create a virtual "html" element to act as the root element, as we won't necessarily
        // have a single root otherwise, while the rest of blitz requires that we do
        let html_element_id = doc.create_node(NodeData::Element(ElementNodeData::new(
            qual_name("html", None),
            Vec::new(),
        )));
        let root_node_id = doc.root_node().id;
        let html_element = doc.get_node_mut(html_element_id).unwrap();
        html_element.parent = Some(root_node_id);
        let root_node = doc.get_node_mut(root_node_id).unwrap();
        root_node.children.push(html_element_id);

        // Include default and user-specified stylesheets
        doc.add_user_agent_stylesheet(DEFAULT_CSS);

        let root_element = doc.root_element().id;

        WebDocument::set_document(doc);

        let local_set = LocalSet::new();
        let (owner, mountable) = local_set.block_on(rt, async { mount_to(root_element.into(), f) });

        Self {
            local_set,
            owner,
            mountable,
        }
    }

    fn inner(&self) -> &'static Document {
        WebDocument::document()
    }
}

impl Drop for LeptosDocument {
    fn drop(&mut self) {
        self.mountable.unmount();
    }
}
