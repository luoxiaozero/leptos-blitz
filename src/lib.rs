mod documents;
mod dom;
mod waker;
mod window;
mod launch;

pub use dom::*;
pub mod prelude {
    pub use leptos::prelude::*;
    pub use crate::dom::IntoView;
    pub use crate::launch::launch;
}

pub use dom::element as html;