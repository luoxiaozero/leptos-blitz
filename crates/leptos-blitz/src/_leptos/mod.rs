mod into_view;
mod mount;

pub use into_view::*;
pub use mount::*;

/// Exports all the core types of the library.
pub mod prelude {
    pub use super::into_view::*;
    pub use crate::_tachys::prelude::*;
    pub use reactive_graph::{
        actions::*, computed::*, effect::*, graph::untrack, owner::*, signal::*, wrappers::read::*,
    };
}

// pub use crate::_tachys as tachys;

/// HTML element types.
pub use crate::_tachys::html::element as html;
