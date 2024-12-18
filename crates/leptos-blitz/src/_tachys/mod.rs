/// Commonly-used traits.
pub mod prelude {
    pub use super::{
        renderer::dom::Dom,
        view::{Mountable, Render, RenderHtml},
    };
}

/// Types for building a statically-typed HTML view tree.
pub mod html;
/// Defines various backends that can render views.
pub mod renderer;
/// Rendering views to HTML.
pub mod ssr;
/// Core logic for manipulating views.
pub mod view;
