mod documents;
mod window;

use documents::{IntoView, LeptosDocument};

pub fn launch<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let docment = LeptosDocument::<N::State>::new(f);

    // let window = crate::window::View::new(document);
    // launch_with_window(window)
}
