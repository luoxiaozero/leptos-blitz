use self::attribute::Attribute;
use crate::tachys::{
    prelude::{AddAnyAttr, Mountable},
    renderer::{
        dom::{Element, Node},
        Rndr,
    },
    view::{Position, Render, RenderHtml},
};
use std::borrow::Cow;

/// Types for HTML attributes.
pub mod attribute;
/// Types for HTML elements.
pub mod element;
/// Types for DOM events.
pub mod event;
/// Types for the `style` attribute and individual style manipulation.
pub mod style;

/// An element that contains no interactivity, and whose contents can be known at compile time.
pub struct InertElement {
    html: Cow<'static, str>,
}

impl InertElement {
    /// Creates a new inert element.
    pub fn new(html: impl Into<Cow<'static, str>>) -> Self {
        Self { html: html.into() }
    }
}

/// Retained view state for [`InertElement`].
pub struct InertElementState(Cow<'static, str>, Element);

impl Mountable for InertElementState {
    fn unmount(&mut self) {
        self.1.unmount();
    }

    fn mount(&mut self, parent: &Element, marker: Option<&Node>) {
        self.1.mount(parent, marker)
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.1.insert_before_this(child)
    }
}

impl Render for InertElement {
    type State = InertElementState;

    fn build(self) -> Self::State {
        let el = Rndr::create_element_from_html(&self.html);
        InertElementState(self.html, el)
    }

    fn rebuild(self, state: &mut Self::State) {
        let InertElementState(prev, el) = state;
        if &self.html != prev {
            let mut new_el = Rndr::create_element_from_html(&self.html);
            el.insert_before_this(&mut new_el);
            el.unmount();
            *el = new_el;
            *prev = self.html;
        }
    }
}

impl AddAnyAttr for InertElement {
    type Output<SomeNewAttr: Attribute> = Self;

    fn add_any_attr<NewAttr: Attribute>(self, _attr: NewAttr) -> Self::Output<NewAttr>
    where
        Self::Output<NewAttr>: RenderHtml,
    {
        panic!(
            "InertElement does not support adding attributes. It should only \
             be used as a child, and not returned at the top level."
        )
    }
}

impl RenderHtml for InertElement {
    type AsyncOutput = Self;

    const MIN_LENGTH: usize = 0;

    fn html_len(&self) -> usize {
        self.html.len()
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self {
        self
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        _escape: bool,
        _mark_branches: bool,
    ) {
        buf.push_str(&self.html);
        *position = Position::NextChild;
    }
}
