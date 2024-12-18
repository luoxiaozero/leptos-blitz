use super::IntoView;
use crate::_tachys::renderer::dom::Element;
use crate::_tachys::view::{Mountable, Render};
use any_spawner::Executor;
use reactive_graph::owner::Owner;

pub fn mount_to<F, N>(parent: Element, f: F) -> (Owner, Box<dyn Mountable>)
where
    F: FnOnce() -> N + 'static,
    N: IntoView + 'static,
{
    _ = Executor::init_tokio();

    let owner = Owner::new();
    let mountable = owner.with(move || {
        let view = f().into_view();
        let mut mountable = view.build();
        mountable.mount(&parent, None);
        Box::new(mountable)
    });

    (owner, mountable)
}
