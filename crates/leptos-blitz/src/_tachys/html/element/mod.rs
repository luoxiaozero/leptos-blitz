mod elements;

pub use elements::*;

use crate::_tachys::{
    renderer::{types, Rndr},
    view::{IntoRender, Mountable, Render},
};
use leptos::{attr::Attribute, html::ElementType};
use next_tuple::NextTuple;
use std::ops::Deref;
#[cfg(debug_assertions)]
use std::panic::Location;

/// The typed representation of an HTML element.
#[derive(Debug, PartialEq, Eq)]
pub struct HtmlElement<E, At, Ch> {
    #[cfg(debug_assertions)]
    pub(crate) defined_at: &'static Location<'static>,
    pub(crate) tag: E,
    pub(crate) attributes: At,
    pub(crate) children: Ch,
}

impl<E: Clone, At: Clone, Ch: Clone> Clone for HtmlElement<E, At, Ch> {
    fn clone(&self) -> Self {
        HtmlElement {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            tag: self.tag.clone(),
            attributes: self.attributes.clone(),
            children: self.children.clone(),
        }
    }
}

impl<E: Copy, At: Copy, Ch: Copy> Copy for HtmlElement<E, At, Ch> {}

impl<E, At, Ch, NewChild> ElementChild<NewChild> for HtmlElement<E, At, Ch>
where
    E: ElementWithChildren,
    Ch: Render + NextTuple,
    <Ch as NextTuple>::Output<NewChild::Output>: Render,

    NewChild: IntoRender,
    NewChild::Output: Render,
{
    type Output = HtmlElement<E, At, <Ch as NextTuple>::Output<NewChild::Output>>;

    fn child(self, child: NewChild) -> Self::Output {
        let HtmlElement {
            #[cfg(debug_assertions)]
            defined_at,
            tag,
            attributes,
            children,
        } = self;
        HtmlElement {
            #[cfg(debug_assertions)]
            defined_at,
            tag,
            attributes,
            children: children.next_tuple(child.into_render()),
        }
    }
}

/// Adds a child to the element.
pub trait ElementChild<NewChild>
where
    NewChild: IntoRender,
{
    /// The type of the element, with the child added.
    type Output;

    /// Adds a child to an element.
    fn child(self, child: NewChild) -> Self::Output;
}

pub(crate) trait ElementWithChildren {}

impl<E, At, Ch> Render for HtmlElement<E, At, Ch>
where
    E: ElementType,
    At: Attribute,
    Ch: Render,
{
    type State = ElementState<At::State, Ch::State>;

    fn rebuild(self, state: &mut Self::State) {
        let ElementState {
            attrs, children, ..
        } = state;
        self.attributes.rebuild(attrs);
        if let Some(children) = children {
            self.children.rebuild(children);
        }
    }

    fn build(self) -> Self::State {
        let el = Rndr::create_element(self.tag.tag(), E::NAMESPACE);

        let attrs = self.attributes.build(&el);
        let children = if E::SELF_CLOSING {
            None
        } else {
            let mut children = self.children.build();
            children.mount(&el, None);
            Some(children)
        };
        ElementState {
            el,
            attrs,
            children,
        }
    }
}

/// The retained view state for an HTML element.
pub struct ElementState<At, Ch> {
    pub(crate) el: types::Element,
    pub(crate) attrs: At,
    pub(crate) children: Option<Ch>,
}

impl<At, Ch> Deref for ElementState<At, Ch> {
    type Target = types::Element;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}

impl<At, Ch> Mountable for ElementState<At, Ch> {
    fn unmount(&mut self) {
        Rndr::remove(self.el.as_ref());
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, self.el.as_ref(), marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        if let Some(parent) = Rndr::get_parent(self.el.as_ref()) {
            if let Some(element) = types::Element::cast_from(parent) {
                child.mount(&element, Some(self.el.as_ref()));
                return true;
            }
        }
        false
    }
}
