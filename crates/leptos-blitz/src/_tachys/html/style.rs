use super::{
    attribute::{Attr, Attribute, AttributeKey, AttributeValue},
    element::HtmlElement,
};
use next_tuple::NextTuple;

impl<E, At, Ch> HtmlElement<E, At, Ch>
where
    At: Attribute,
{
    pub fn style<V>(self, value: V) -> HtmlElement<E, <At as NextTuple>::Output<Attr<Style, V>>, Ch>
    where
        V: AttributeValue,
        At: NextTuple,
        <At as NextTuple>::Output<Attr<Style, V>>: Attribute,
    {
        let HtmlElement {
            tag,
            children,
            attributes,
            #[cfg(debug_assertions)]
            defined_at,
        } = self;
        HtmlElement {
            tag,
            children,
            attributes: attributes.next_tuple(style(value)),
            #[cfg(debug_assertions)]
            defined_at,
        }
    }
}

fn style<V>(value: V) -> Attr<Style, V>
where
    V: AttributeValue,
{
    Attr(Style, value)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Style;

impl AttributeKey for Style {
    const KEY: &'static str = "style";
}
