mod into_view;
mod mount;

pub use into_view::*;
pub use mount::*;

/// Exports all the core types of the library.
pub mod prelude {
    pub use super::into_view::*;
    pub use crate::_tachys::prelude::*;
}

/// HTML element types.
#[doc(inline)]
pub use crate::_tachys::html::element as html;
