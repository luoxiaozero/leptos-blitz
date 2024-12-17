/// Commonly-used traits.
pub mod prelude {
    pub use super::{
        renderer::dom::Dom,
        view::{Mountable, Render},
    };
}

/// Types for building a statically-typed HTML view tree.
pub mod html;
/// Defines various backends that can render views.
pub mod renderer;

pub use renderer::dom::Dom;
/// Core logic for manipulating views.
pub mod view;
