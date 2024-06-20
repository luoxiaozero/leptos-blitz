mod documents;
mod dom;
mod launch;
mod waker;
mod window;

pub mod prelude {
    pub use crate::dom::IntoView;
    pub use crate::launch::launch;
    pub use leptos::prelude::*;
}

pub use dom::html::element as html;
pub use leptos;
