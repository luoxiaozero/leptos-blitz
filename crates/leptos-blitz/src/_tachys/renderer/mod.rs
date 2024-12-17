pub mod dom;

pub mod types {
    pub use super::dom::{Element, Node};
}

pub type Rndr = dom::Dom;