mod elements;

pub use elements::*;

use crate::_tachys::{
    html::attribute::{escape_attr, Attribute, NextAttribute},
    renderer::{types, CastFrom, Rndr},
    ssr::StreamBuilder,
    view::{add_attr::AddAnyAttr, IntoRender, Mountable, Position, Render, RenderHtml},
};
use futures::future::join;
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

impl<E, At, Ch> AddAnyAttr for HtmlElement<E, At, Ch>
where
    E: ElementType + Send,
    At: Attribute + Send,
    Ch: RenderHtml + Send,
{
    type Output<SomeNewAttr: Attribute> =
        HtmlElement<E, <At as NextAttribute>::Output<SomeNewAttr>, Ch>;

    fn add_any_attr<NewAttr: Attribute>(self, attr: NewAttr) -> Self::Output<NewAttr> {
        let HtmlElement {
            #[cfg(any(debug_assertions, leptos_debuginfo))]
            defined_at,
            tag,
            attributes,
            children,
        } = self;
        HtmlElement {
            #[cfg(any(debug_assertions, leptos_debuginfo))]
            defined_at,
            tag,
            attributes: attributes.add_any_attr(attr),
            children,
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

/// An HTML element.
pub trait ElementType: Send {
    /// The underlying native widget type that this represents.
    type Output;

    /// The element's tag.
    const TAG: &'static str;
    /// Whether the element is self-closing.
    const SELF_CLOSING: bool;
    /// Whether the element's children should be escaped. This should be `true` except for elements
    /// like `<style>` and `<script>`, which include other languages that should not use HTML
    /// entity escaping.
    const ESCAPE_CHILDREN: bool;
    /// The element's namespace, if it is not HTML.
    const NAMESPACE: Option<&'static str>;

    /// The element's tag.
    fn tag(&self) -> &str;
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

impl<E, At, Ch> RenderHtml for HtmlElement<E, At, Ch>
where
    E: ElementType + Send,
    At: Attribute + Send,
    Ch: RenderHtml + Send,
{
    type AsyncOutput = HtmlElement<E, At::AsyncOutput, Ch::AsyncOutput>;

    const MIN_LENGTH: usize = if E::SELF_CLOSING {
        3 // < ... />
        + E::TAG.len()
        + At::MIN_LENGTH
    } else {
        2 // < ... >
        + E::TAG.len()
        + At::MIN_LENGTH
        + Ch::MIN_LENGTH
        + 3 // </ ... >
        + E::TAG.len()
    };

    fn dry_resolve(&mut self) {
        self.attributes.dry_resolve();
        self.children.dry_resolve();
    }

    async fn resolve(self) -> Self::AsyncOutput {
        let (attributes, children) = join(self.attributes.resolve(), self.children.resolve()).await;
        HtmlElement {
            #[cfg(any(debug_assertions, leptos_debuginfo))]
            defined_at: self.defined_at,
            tag: self.tag,
            attributes,
            children,
        }
    }

    fn html_len(&self) -> usize {
        if E::SELF_CLOSING {
            3 // < ... />
        + E::TAG.len()
        + self.attributes.html_len()
        } else {
            2 // < ... >
        + E::TAG.len()
        + self.attributes.html_len()
        + self.children.html_len()
        + 3 // </ ... >
        + E::TAG.len()
        }
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        _escape: bool,
        mark_branches: bool,
    ) {
        // opening tag
        buf.push('<');
        buf.push_str(self.tag.tag());

        let inner_html = attributes_to_html(self.attributes, buf);

        buf.push('>');

        if !E::SELF_CLOSING {
            if !inner_html.is_empty() {
                buf.push_str(&inner_html);
            } else if Ch::EXISTS {
                // children
                *position = Position::FirstChild;
                self.children
                    .to_html_with_buf(buf, position, E::ESCAPE_CHILDREN, mark_branches);
            }

            // closing tag
            buf.push_str("</");
            buf.push_str(self.tag.tag());
            buf.push('>');
        }
        *position = Position::NextChild;
    }

    fn to_html_async_with_buf<const OUT_OF_ORDER: bool>(
        self,
        buffer: &mut StreamBuilder,
        position: &mut Position,
        _escape: bool,
        mark_branches: bool,
    ) where
        Self: Sized,
    {
        let mut buf = String::with_capacity(Self::MIN_LENGTH);
        // opening tag
        buf.push('<');
        buf.push_str(self.tag.tag());

        let inner_html = attributes_to_html(self.attributes, &mut buf);

        buf.push('>');
        buffer.push_sync(&buf);

        if !E::SELF_CLOSING {
            // children
            *position = Position::FirstChild;
            if !inner_html.is_empty() {
                buffer.push_sync(&inner_html);
            } else if Ch::EXISTS {
                self.children.to_html_async_with_buf::<OUT_OF_ORDER>(
                    buffer,
                    position,
                    E::ESCAPE_CHILDREN,
                    mark_branches,
                );
            }

            // closing tag
            let mut buf = String::with_capacity(3 + E::TAG.len());
            buf.push_str("</");
            buf.push_str(self.tag.tag());
            buf.push('>');
            buffer.push_sync(&buf);
        }
        *position = Position::NextChild;
    }
}

/// Renders an [`Attribute`] (which can be one or more HTML attributes) into an HTML buffer.
pub fn attributes_to_html<At>(attr: At, buf: &mut String) -> String
where
    At: Attribute,
{
    // `class` and `style` are created first, and pushed later
    // this is because they can be filled by a mixture of values that include
    // either the whole value (`class="..."` or `style="..."`) and individual
    // classes and styles (`class:foo=true` or `style:height="40px"`), so they
    // need to be filled during the whole attribute-creation process and then
    // added

    // String doesn't allocate until the first push, so this is cheap if there
    // is no class or style on an element
    let mut class = String::new();
    let mut style = String::new();
    let mut inner_html = String::new();

    // inject regular attributes, and fill class and style
    attr.to_html(buf, &mut class, &mut style, &mut inner_html);

    if !class.is_empty() {
        buf.push(' ');
        buf.push_str("class=\"");
        buf.push_str(&escape_attr(class.trim_start().trim_end()));
        buf.push('"');
    }
    if !style.is_empty() {
        buf.push(' ');
        buf.push_str("style=\"");
        buf.push_str(&escape_attr(style.trim_start().trim_end()));
        buf.push('"');
    }

    inner_html
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
        Rndr::remove(&self.el);
    }

    fn mount(&mut self, parent: &types::Element, marker: Option<&types::Node>) {
        Rndr::insert_node(parent, &self.el, marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        if let Some(parent) = Rndr::get_parent(&self.el) {
            if let Some(element) = types::Element::cast_from(parent) {
                child.mount(&element, Some(&self.el));
                return true;
            }
        }
        false
    }
}
