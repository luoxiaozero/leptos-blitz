mod value;

use crate::_tachys::renderer::types;
use std::future::Future;
pub use value::*;
/// Defines an attribute: anything that can modify an element.
pub trait Attribute: NextAttribute + Send {
    /// The minimum length of this attribute in HTML.
    const MIN_LENGTH: usize;

    /// The state that should be retained between building and rebuilding.
    type State;
    /// The type once all async data have loaded.
    type AsyncOutput: Attribute;
    /// An equivalent to this attribute that can be cloned to be shared across elements.
    type Cloneable: Attribute + Clone;
    /// An equivalent to this attribute that can be cloned to be shared across elements, and
    /// captures no references shorter than `'static`.
    type CloneableOwned: Attribute + Clone + 'static;

    /// An approximation of the actual length of this attribute in HTML.
    fn html_len(&self) -> usize;

    /// Renders the attribute to HTML.
    ///
    /// This separates a general buffer for attribute values from the `class` and `style`
    /// attributes, so that multiple classes or styles can be combined, and also allows for an
    /// `inner_html` attribute that sets the child HTML instead of an attribute.
    fn to_html(
        self,
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
    );

    /// Adds interactivity as necessary, given DOM nodes that were created from HTML that has
    /// either been rendered on the server, or cloned for a `<template>`.
    fn hydrate<const FROM_SERVER: bool>(self, el: &types::Element) -> Self::State;

    /// Adds this attribute to the element during client-side rendering.
    fn build(self, el: &types::Element) -> Self::State;

    /// Applies a new value for the attribute.
    fn rebuild(self, state: &mut Self::State);

    /// Converts this attribute into an equivalent that can be cloned.
    fn into_cloneable(self) -> Self::Cloneable;

    /// Converts this attributes into an equivalent that can be cloned and is `'static`.
    fn into_cloneable_owned(self) -> Self::CloneableOwned;

    /// “Runs” the attribute without other side effects. For primitive types, this is a no-op. For
    /// reactive types, this can be used to gather data about reactivity or about asynchronous data
    /// that needs to be loaded.
    fn dry_resolve(&mut self);

    /// “Resolves” this into a type that is not waiting for any asynchronous data.
    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send;
}

pub trait NextAttribute {
    /// The type of the new, combined attribute.
    type Output<NewAttr: Attribute>: Attribute;

    /// Adds a new attribute.
    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr>;
}
