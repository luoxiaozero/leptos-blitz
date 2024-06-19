use crate::dom::{Element, IntoView, LeptosDom, Node};
use any_spawner::Executor;
use blitz::Viewport;
use blitz_dom::{Document, DocumentLike};
use leptos::{
    reactive_graph::owner::Owner,
    tachys::view::{Mountable, Render},
};
use std::cell::RefCell;

thread_local! {
    static DOCUMENT: RefCell<Option<Document>> = RefCell::new(None);
}

pub struct LeptosDocument {
    #[allow(dead_code)]
    unmount_handle: UnmountHandle,
}

impl AsRef<Document> for LeptosDocument {
    fn as_ref(&self) -> &Document {
        LeptosDocument::document()
    }
}
impl AsMut<Document> for LeptosDocument {
    fn as_mut(&mut self) -> &mut Document {
        LeptosDocument::document_mut()
    }
}
impl Into<Document> for LeptosDocument {
    fn into(self) -> Document {
        LeptosDocument::document_take()
    }
}

impl DocumentLike for LeptosDocument {
    fn poll(&mut self, _cx: std::task::Context) -> bool {
        // TODO
        // loop {
        //     {
        //         // pin_mut!(fut);

        //         // match fut.poll_unpin(&mut cx) {
        //         //     std::task::Poll::Ready(_) => {}
        //         //     std::task::Poll::Pending => break,
        //         // }
        //     }
        // }
        false
    }

    fn handle_event(&mut self, event: blitz_dom::events::RendererEvent) -> bool {
        // Collect the nodes into a chain by traversing upwards
        // This is important so the "capture" phase can be implemented
        let mut node_id = event.target;
        let mut chain = Vec::with_capacity(16);
        chain.push(node_id);

        // // if it's a capturing event, we want to fill in the chain with the parent nodes
        // // until we reach the root - that way we can call the listeners in the correct order
        // // otherwise, we just want to call the listeners on the target
        // //
        // // todo: this is harcoded for "click" events - eventually we actually need to handle proper propagation
        // // if event.name == "click" {
        // while let Some(parent) = self.inner.tree()[node_id].parent {
        //     chain.push(parent);
        //     node_id = parent;
        // }

        // set_event_converter(Box::new(NativeConverter {}));

        // // look for the data-dioxus-id attribute on the element
        // // todo: we might need to walk upwards to find the first element with a data-dioxus-id attribute
        // for node in chain.iter() {
        //     let Some(element) = self.inner.tree()[*node].element_data() else {
        //         println!(
        //             "No element data found for node {}: {:?}",
        //             node,
        //             self.inner.tree()[*node]
        //         );
        //         continue;
        //     };

        //     for attr in element.attrs() {
        //         if attr.name.local.as_ref() == "data-dioxus-id" {
        //             if let Ok(value) = attr.value.parse::<usize>() {
        //                 let id = ElementId(value);
        //                 // let data = dioxus::html::EventData::Mouse()

        //                 let data = Rc::new(PlatformEventData::new(Box::new(NativeClickData {})));
        //                 self.vdom.handle_event("click", data, id, true);
        //                 return true;
        //             }
        //         }
        //     }
        // }

        false
    }
}

impl LeptosDocument {
    pub fn new<F, N>(f: F) -> LeptosDocument
    where
        F: FnOnce() -> N + 'static,
        N: IntoView + 'static,
    {
        DOCUMENT.with(|doc| {
            let device = Viewport::new((0, 0)).make_device();
            let mut document = Document::new(device);
            document.add_stylesheet(include_str!("./default.css"));

            *doc.borrow_mut() = Some(document);
        });

        let unmount_handle = LeptosDocument::mount_to(f);

        LeptosDocument::document().print_tree();

        LeptosDocument { unmount_handle }
    }

    fn mount_to<F, N>(f: F) -> UnmountHandle
    where
        F: FnOnce() -> N + 'static,
        N: IntoView + 'static,
    {
        _ = Executor::init_tokio();

        let owner = Owner::new();
        let mountable = owner.with(move || {
            let view = f().into_view();
            let mut mountable = view.build();

            let root_node_id = LeptosDocument::document().root_node().id;

            mountable.mount(&Element(Node(root_node_id)), None);

            Box::new(mountable) as Box<dyn Mountable<LeptosDom>>
        });

        UnmountHandle { owner, mountable }
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

pub struct UnmountHandle {
    #[allow(dead_code)]
    owner: Owner,
    mountable: Box<dyn Mountable<LeptosDom>>,
}

impl Drop for UnmountHandle {
    fn drop(&mut self) {
        self.mountable.unmount();
    }
}
