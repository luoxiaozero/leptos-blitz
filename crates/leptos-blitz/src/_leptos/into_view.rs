use crate::_tachys::prelude::{Render, RenderHtml};
use std::borrow::Cow;

/// A wrapper for any kind of view.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct View<T>
where
    T: Sized,
{
    inner: T,
    #[cfg(debug_assertions)]
    view_marker: Option<Cow<'static, str>>,
}

pub trait IntoView
where
    Self: Sized + Render + RenderHtml + Send,
{
    /// Wraps the inner type.
    fn into_view(self) -> View<Self>;
}

impl<T: Render> Render for View<T> {
    type State = T::State;

    fn build(self) -> Self::State {
        self.inner.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.inner.rebuild(state);
    }
}
