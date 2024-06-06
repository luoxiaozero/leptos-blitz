use any_spawner::Executor;
use blitz::Viewport;
use blitz_dom::Document;
use leptos::{mount::mount_to_body, reactive_graph::owner::Owner, IntoView};

pub struct LeptosDocument {
    inner: Document,
}

impl LeptosDocument {
    pub fn new() -> Self {
        let device = Viewport::new((0, 0)).make_device();
        let mut doc = Document::new(device);

        Self { inner: doc }
    }

    pub fn mount_to_root_node<F, N>(&self, f: F)
    where
        F: FnOnce() -> N + 'static,
        N: IntoView,
    {
        let parent = self.inner.root_node();

        _ = Executor::init_tokio();

        let owner = Owner::new();
        owner.p
        let mountable = owner.with(move || {
            let view = f().into_view();
            let mut mountable = view.build();
            mountable.mount(&parent, None);
            mountable
        });

        mount_to_body(f)
    }
}
