use crate::dom::{Element, IntoView, LeptosDom};
use any_spawner::Executor;
use blitz::Viewport;
use blitz_dom::Document;
use leptos::{
    context::{provide_context, use_context},
    reactive_graph::owner::Owner,
    tachys::view::{Mountable, Render},
};
use send_wrapper::SendWrapper;
use std::sync::Arc;

pub struct LeptosDocument {
    inner: Arc<Document>,
    unmount_handle: UnmountHandle,
}

impl LeptosDocument {
    pub fn new<F, N>(f: F) -> LeptosDocument
    where
        F: FnOnce() -> N + 'static,
        N: IntoView + 'static,
    {
        let device = Viewport::new((0, 0)).make_device();
        let document = Arc::new(Document::new(device));

        let unmount_handle = LeptosDocument::mount_to(SendWrapper::new(document.clone()), f);

        LeptosDocument {
            inner: document,
            unmount_handle,
        }
    }

    fn mount_to<F, N>(document: SendWrapper<Arc<Document>>, f: F) -> UnmountHandle
    where
        F: FnOnce() -> N + 'static,
        N: IntoView + 'static,
    {
        _ = Executor::init_tokio();

        let owner = Owner::new();
        let mountable = owner.with(move || {
            provide_context(document.clone());

            let view = f().into_view();
            let mut mountable = view.build();

            let parent = document.root_node();
            let parent = unsafe { Element::convert_from_node(parent) };

            mountable.mount(parent, None);

            Box::new(mountable) as Box<dyn Mountable<LeptosDom>>
        });

        UnmountHandle { owner, mountable }
    }

    pub fn use_document() -> SendWrapper<Arc<Document>> {
        use_context().expect("SendWrapper<Arc<Document>>")
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
