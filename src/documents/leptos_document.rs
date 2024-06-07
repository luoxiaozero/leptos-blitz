use std::sync::Arc;

use super::leptos_dom::{Element, IntoView, LeptosDom};
use any_spawner::Executor;
use blitz::Viewport;
use blitz_dom::Document;
use leptos::{
    context::use_context, reactive_graph::owner::Owner, tachys::view::{Mountable, Render}
};

pub struct LeptosDocument<M>
where
    M: Mountable<LeptosDom>,
{
    inner: Document,
    unmount_handle: UnmountHandle<M>,
}

impl<M> LeptosDocument<M>
where
    M: Mountable<LeptosDom>,
{
    pub fn new<F, N>(f: F) -> LeptosDocument<N::State>
    where
        F: FnOnce() -> N + 'static,
        N: IntoView,
    {
        let device = Viewport::new((0, 0)).make_device();
        let document = Document::new(device);

        let unmount_handle = LeptosDocument::<N::State>::mount_to(&document, f);

        LeptosDocument {
            inner: document,
            unmount_handle,
        }
    }

    fn mount_to<F, N>(document: &Document, f: F) -> UnmountHandle<N::State>
    where
        F: FnOnce() -> N + 'static,
        N: IntoView,
    {
        let parent = document.root_node();
        let parent = unsafe { Element::convert_from_node(parent) };

        _ = Executor::init_tokio();

        let owner = Owner::new();
        let mountable = owner.with(move || {
            let view = f().into_view();
            let mut mountable = view.build();
            mountable.mount(parent, None);

            mountable
        });

        UnmountHandle { owner, mountable }
    }

    pub fn use_document() -> Arc<Document> {
        use_context().expect("Docunebt ")
    }
}

pub struct UnmountHandle<M>
where
    M: Mountable<LeptosDom>,
{
    #[allow(dead_code)]
    owner: Owner,
    mountable: M,
}

impl<M> Drop for UnmountHandle<M>
where
    M: Mountable<LeptosDom>,
{
    fn drop(&mut self) {
        self.mountable.unmount();
    }
}
