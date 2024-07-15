mod documents;
pub mod dom;
mod launch;
mod waker;
mod window;

#[cfg(feature = "accessibility")]
mod accessibility;

pub mod prelude {
    pub use crate::dom::IntoView;
    pub use crate::launch::launch;
    pub use leptos::prelude::*;
    pub use leptos_blitz_macro::view;
}

pub use dom::html::element as html;
pub use dom::html::event as ev;
pub use leptos;
