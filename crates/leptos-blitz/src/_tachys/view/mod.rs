use self::add_attr::AddAnyAttr;
use crate::_tachys::{renderer::types, ssr::StreamBuilder};
use parking_lot::RwLock;
use std::{future::Future, sync::Arc};

/// Add attributes to typed views.
pub mod add_attr;
mod primitives;
/// View implementation for string types.
pub mod strings;
/// View implementations for tuples.
pub mod tuples;

/// The `Render` trait allows rendering something as part of the user interface.
pub trait Render: Sized {
    /// The “view state” for this type, which can be retained between updates.
    ///
    /// For example, for a text node, `State` might be the actual DOM text node
    /// and the previous string, to allow for diffing between updates.
    type State: Mountable;

    /// Creates the view for the first time, without hydrating from existing HTML.
    fn build(self) -> Self::State;

    /// Updates the view with new data.
    fn rebuild(self, state: &mut Self::State);
}

/// The `RenderHtml` trait allows rendering something to HTML, and transforming
/// that HTML into an interactive interface.
///
/// This process is traditionally called “server rendering” and “hydration.” As a
/// metaphor, this means that the structure of the view is created on the server, then
/// “dehydrated” to HTML, sent across the network, and “rehydrated” with interactivity
/// in the browser.
///
/// However, the same process can be done entirely in the browser: for example, a view
/// can be transformed into some HTML that is used to create a `<template>` node, which
/// can be cloned many times and “hydrated,” which is more efficient than creating the
/// whole view piece by piece.
pub trait RenderHtml
where
    Self: Render + AddAnyAttr + Send,
{
    /// The type of the view after waiting for all asynchronous data to load.
    type AsyncOutput: RenderHtml;

    /// The minimum length of HTML created when this view is rendered.
    const MIN_LENGTH: usize;

    /// Whether this should actually exist in the DOM, if it is the child of an element.
    const EXISTS: bool = true;

    /// “Runs” the view without other side effects. For primitive types, this is a no-op. For
    /// reactive types, this can be used to gather data about reactivity or about asynchronous data
    /// that needs to be loaded.
    fn dry_resolve(&mut self);

    /// Waits for any asynchronous sections of the view to load and returns the output.
    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send;

    /// An estimated length for this view, when rendered to HTML.
    ///
    /// This is used for calculating the string buffer size when rendering HTML. It does not need
    /// to be precise, but should be an appropriate estimate. The more accurate, the fewer
    /// reallocations will be required and the faster server-side rendering will be.
    fn html_len(&self) -> usize {
        Self::MIN_LENGTH
    }

    /// Renders a view to an HTML string.
    fn to_html(self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::with_capacity(self.html_len());
        self.to_html_with_buf(&mut buf, &mut Position::FirstChild, true, false);
        buf
    }

    /// Renders a view to HTML with branch markers. This can be used to support libraries that diff
    /// HTML pages against one another, by marking sections of the view that branch to different
    /// types with marker comments.
    fn to_html_branching(self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::with_capacity(self.html_len());
        self.to_html_with_buf(&mut buf, &mut Position::FirstChild, true, true);
        buf
    }

    /// Renders a view to an in-order stream of HTML.
    fn to_html_stream_in_order(self) -> StreamBuilder
    where
        Self: Sized,
    {
        let mut builder = StreamBuilder::with_capacity(self.html_len(), None);
        self.to_html_async_with_buf::<false>(&mut builder, &mut Position::FirstChild, true, false);
        builder.finish()
    }

    /// Renders a view to an in-order stream of HTML with branch markers. This can be used to support libraries that diff
    /// HTML pages against one another, by marking sections of the view that branch to different
    /// types with marker comments.
    fn to_html_stream_in_order_branching(self) -> StreamBuilder
    where
        Self: Sized,
    {
        let mut builder = StreamBuilder::with_capacity(self.html_len(), None);
        self.to_html_async_with_buf::<false>(&mut builder, &mut Position::FirstChild, true, true);
        builder.finish()
    }

    /// Renders a view to an out-of-order stream of HTML.
    fn to_html_stream_out_of_order(self) -> StreamBuilder
    where
        Self: Sized,
    {
        //let capacity = self.html_len();
        let mut builder = StreamBuilder::with_capacity(self.html_len(), Some(vec![0]));

        self.to_html_async_with_buf::<true>(&mut builder, &mut Position::FirstChild, true, false);
        builder.finish()
    }

    /// Renders a view to an out-of-order stream of HTML with branch markers. This can be used to support libraries that diff
    /// HTML pages against one another, by marking sections of the view that branch to different
    /// types with marker comments.
    fn to_html_stream_out_of_order_branching(self) -> StreamBuilder
    where
        Self: Sized,
    {
        let mut builder = StreamBuilder::with_capacity(self.html_len(), Some(vec![0]));

        self.to_html_async_with_buf::<true>(&mut builder, &mut Position::FirstChild, true, true);
        builder.finish()
    }

    /// Renders a view to HTML, writing it into the given buffer.
    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    );

    /// Renders a view into a buffer of (synchronous or asynchronous) HTML chunks.
    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buf: &mut StreamBuilder,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
    ) where
        Self: Sized,
    {
        buf.with_buf(|buf| self.to_html_with_buf(buf, position, escape, mark_branches));
    }
}

/// Allows a type to be mounted to the DOM.
pub trait Mountable {
    /// Detaches the view from the DOM.
    fn unmount(&mut self);

    /// Mounts a node to the interface.
    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>);

    /// Inserts another `Mountable` type before this one. Returns `false` if
    /// this does not actually exist in the UI (for example, `()`).
    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool;

    /// Inserts another `Mountable` type before this one, or before the marker
    /// if this one doesn't exist in the UI (for example, `()`).
    fn insert_before_this_or_marker(
        &self,
        parent: &types::Element,
        child: &mut dyn Mountable,
        marker: Option<&types::Node>,
    ) {
        if !self.insert_before_this(child) {
            child.mount(parent, marker);
        }
    }
}

/// Keeps track of what position the item currently being hydrated is in, relative to its siblings
/// and parents.
#[derive(Debug, Default, Clone)]
pub struct PositionState(Arc<RwLock<Position>>);

impl PositionState {
    /// Creates a new position tracker.
    pub fn new(position: Position) -> Self {
        Self(Arc::new(RwLock::new(position)))
    }

    /// Sets the current position.
    pub fn set(&self, position: Position) {
        *self.0.write() = position;
    }

    /// Gets the current position.
    pub fn get(&self) -> Position {
        *self.0.read()
    }

    /// Creates a new [`PositionState`], which starts with the same [`Position`], but no longer
    /// shares data with this `PositionState`.
    pub fn deep_clone(&self) -> Self {
        let current = self.get();
        Self(Arc::new(RwLock::new(current)))
    }
}

/// The position of this element, relative to others.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Position {
    /// This is the current node.
    Current,
    /// This is the first child of its parent.
    #[default]
    FirstChild,
    /// This is the next child after another child.
    NextChild,
    /// This is the next child after a text node.
    NextChildAfterText,
    /// This is the only child of its parent.
    OnlyChild,
    /// This is the last child of its parent.
    LastChild,
}

/// Declares that this type can be converted into some other type, which can be renderered.
pub trait IntoRender {
    /// The renderable type into which this type can be converted.
    type Output;

    /// Consumes this value, transforming it into the renderable type.
    fn into_render(self) -> Self::Output;
}

impl<T> IntoRender for T
where
    T: Render,
{
    type Output = Self;

    fn into_render(self) -> Self::Output {
        self
    }
}
