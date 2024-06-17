use leptos::{html::CreateElement, tachys::{
    html::attribute::{Attribute, NextAttribute},
    renderer::{CastFrom, Renderer},
    view::{add_attr::AddAnyAttr, Mountable, Render, RenderHtml},
}};
use next_tuple::NextTuple;
use std::{marker::PhantomData, ops::Deref};

mod elements;
pub use elements::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HtmlElement<E, At, Ch, Rndr> {
    pub(crate) tag: E,
    pub(crate) rndr: PhantomData<Rndr>,
    pub(crate) attributes: At,
    pub(crate) children: Ch,
    #[cfg(debug_assertions)]
    pub(crate) defined_at: &'static std::panic::Location<'static>,
}

impl<E, At, Ch, Rndr> HtmlElement<E, At, Ch, Rndr> {
    pub fn children(&self) -> &Ch {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Ch {
        &mut self.children
    }

    pub fn attributes(&self) -> &At {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut At {
        &mut self.attributes
    }
}

impl<E, At, Ch, NewChild, Rndr> ElementChild<Rndr, NewChild> for HtmlElement<E, At, Ch, Rndr>
where
    E: ElementWithChildren,
    Ch: Render<Rndr> + NextTuple,
    <Ch as NextTuple>::Output<NewChild>: Render<Rndr>,
    Rndr: Renderer,
    NewChild: Render<Rndr>,
{
    type Output = HtmlElement<E, At, <Ch as NextTuple>::Output<NewChild>, Rndr>;

    fn child(self, child: NewChild) -> Self::Output {
        let HtmlElement {
            tag,
            rndr,
            attributes,
            children,
            #[cfg(debug_assertions)]
            defined_at,
        } = self;
        HtmlElement {
            tag,
            rndr,
            attributes,
            children: children.next_tuple(child),
            #[cfg(debug_assertions)]
            defined_at,
        }
    }
}

// impl<E, At, Ch, Rndr> AddAnyAttr<Rndr> for HtmlElement<E, At, Ch, Rndr>
// where
//     E: ElementType + CreateElement<Rndr> + Send,
//     At: Attribute<Rndr> + Send,
//     Ch: RenderHtml<Rndr> + Send,
//     Rndr: Renderer,
// {
//     type Output<SomeNewAttr: Attribute<Rndr>> =
//         HtmlElement<E, <At as NextAttribute<Rndr>>::Output<SomeNewAttr>, Ch, Rndr>;

//     fn add_any_attr<NewAttr: Attribute<Rndr>>(self, attr: NewAttr) -> Self::Output<NewAttr> {
//         let HtmlElement {
//             tag,
//             attributes,
//             children,
//             rndr,
//             #[cfg(debug_assertions)]
//             defined_at,
//         } = self;
//         HtmlElement {
//             tag,
//             attributes: attributes.add_any_attr(attr),
//             children,
//             rndr,
//             #[cfg(debug_assertions)]
//             defined_at,
//         }
//     }
// }

pub trait ElementChild<Rndr, NewChild>
where
    NewChild: Render<Rndr>,
    Rndr: Renderer,
{
    type Output;

    fn child(self, child: NewChild) -> Self::Output;
}

pub trait ElementType: Send {
    /// The underlying native widget type that this represents.
    type Output;

    const TAG: &'static str;
    const SELF_CLOSING: bool;
    const ESCAPE_CHILDREN: bool;

    fn tag(&self) -> &str;
}

pub trait HasElementType {
    type ElementType;
}

pub trait ElementWithChildren {}

impl<E, At, Ch, Rndr> HasElementType for HtmlElement<E, At, Ch, Rndr>
where
    E: ElementType,
{
    type ElementType = E::Output;
}

impl<E, At, Ch, Rndr> Render<Rndr> for HtmlElement<E, At, Ch, Rndr>
where
    E: CreateElement<Rndr>,
    At: Attribute<Rndr>,
    Ch: Render<Rndr>,
    Rndr: Renderer,
{
    type State = ElementState<At::State, Ch::State, Rndr>;

    fn rebuild(self, state: &mut Self::State) {
        let ElementState {
            attrs, children, ..
        } = state;
        self.attributes.rebuild(attrs);
        self.children.rebuild(children);
    }

    fn build(self) -> Self::State {
        let el = Rndr::create_element(self.tag);

        let attrs = self.attributes.build(&el);
        let mut children = self.children.build();
        children.mount(&el, None);
        ElementState {
            el,
            attrs,
            children,
            rndr: PhantomData,
        }
    }
}

pub struct ElementState<At, Ch, R: Renderer> {
    pub el: R::Element,
    pub attrs: At,
    pub children: Ch,
    rndr: PhantomData<R>,
}

impl<At, Ch, R: Renderer> Deref for ElementState<At, Ch, R> {
    type Target = R::Element;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}

impl<At, Ch, R> Mountable<R> for ElementState<At, Ch, R>
where
    R: Renderer,
{
    fn unmount(&mut self) {
        R::remove(self.el.as_ref());
    }

    fn mount(&mut self, parent: &R::Element, marker: Option<&R::Node>) {
        R::insert_node(parent, self.el.as_ref(), marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable<R>) -> bool {
        if let Some(parent) = R::get_parent(self.el.as_ref()) {
            if let Some(element) = R::Element::cast_from(parent) {
                child.mount(&element, Some(self.el.as_ref()));
                return true;
            }
        }
        false
    }
}
