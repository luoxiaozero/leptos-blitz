/// Commonly-used traits.
pub mod prelude {
    pub use super::{
        renderer::dom::Dom,
        view::{Mountable, Render},
    };
}

pub mod renderer;

pub use renderer::dom::Dom;
/// Core logic for manipulating views.
pub mod view;
