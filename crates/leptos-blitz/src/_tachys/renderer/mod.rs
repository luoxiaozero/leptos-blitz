pub mod dom;

pub mod types {
    pub use super::dom::{Element, Node, Placeholder, Text};
}

pub type Rndr = dom::Dom;

pub trait CastFrom<T>
where
    Self: Sized,
{
    /// Casts a node from one type to another.
    fn cast_from(source: T) -> Option<Self>;
}
