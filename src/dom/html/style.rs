use super::{attribute::Attr, element::HtmlElement};
use leptos::tachys::{
    html::attribute::{Attribute, AttributeKey, AttributeValue},
    renderer::Renderer,
};
use next_tuple::NextTuple;
use std::marker::PhantomData;

impl<E, At, Rndr> HtmlElement<E, At, (), Rndr>
where
    At: Attribute<Rndr>,
    Rndr: Renderer,
{
    pub fn style<V>(
        self,
        value: V,
    ) -> HtmlElement<E, <At as NextTuple>::Output<Attr<Style, V, Rndr>>, (), Rndr>
    where
        V: AttributeValue<Rndr>,
        At: NextTuple,
        <At as NextTuple>::Output<Attr<Style, V, Rndr>>: Attribute<Rndr>,
    {
        let HtmlElement {
            tag,
            rndr,
            children,
            attributes,
            #[cfg(debug_assertions)]
            defined_at,
        } = self;
        HtmlElement {
            tag,
            rndr,
            children,
            attributes: attributes.next_tuple(style(value)),
            #[cfg(debug_assertions)]
            defined_at,
        }
    }
}

pub fn style<V, Rndr>(value: V) -> Attr<Style, V, Rndr>
where
    V: AttributeValue<Rndr>,
    Rndr: Renderer,
{
    Attr(Style, value, PhantomData)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Style;

impl AttributeKey for Style {
    const KEY: &'static str = "style";
}
