use crate::_tachys::renderer::types;

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
