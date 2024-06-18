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
    fn poll(&mut self, _cx: std::task::Context) {
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
            let document = Document::new(device);
            *doc.borrow_mut() = Some(document);
        });

        let unmount_handle = LeptosDocument::mount_to(f);

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
