use super::super::element::HtmlElement;
use crate::dom::html::event::{on, Event, EventDescriptor, On};
use leptos::tachys::{html::attribute::Attribute, renderer::Renderer};
use next_tuple::NextTuple;

impl<E, At, Rndr> HtmlElement<E, At, (), Rndr>
where
    At: Attribute<Rndr>,
    Rndr: Renderer,
{
    pub fn on<EV, F>(
        self,
        event: EV,
        cb: F,
    ) -> HtmlElement<E, <At as NextTuple>::Output<On<EV, F, Rndr>>, (), Rndr>
    where
        EV: EventDescriptor + Send + 'static,
        EV::EventType: 'static,
        EV::EventType: From<Event>,
        F: FnMut(EV::EventType) + 'static,
        At: NextTuple,
        <At as NextTuple>::Output<On<EV, F, Rndr>>: Attribute<Rndr>,
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
            attributes: attributes.next_tuple(on(event, cb)),
            #[cfg(debug_assertions)]
            defined_at,
        }
    }
}
