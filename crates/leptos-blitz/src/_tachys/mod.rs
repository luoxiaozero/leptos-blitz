/// Commonly-used traits.
pub mod prelude {
    pub use super::{
        html::element::ElementChild,
        renderer::dom::Dom,
        view::{add_attr::AddAnyAttr, IntoRender, Mountable, Render, RenderHtml},
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

pub mod reactive_graph;
