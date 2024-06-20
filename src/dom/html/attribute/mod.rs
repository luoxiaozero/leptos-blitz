pub mod on;

use leptos::tachys::{
    html::attribute::{Attribute, AttributeKey, AttributeValue, NextAttribute},
    renderer::Renderer,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Attr<K, V, R>(pub K, pub V, pub PhantomData<R>)
where
    K: AttributeKey,
    V: AttributeValue<R>,
    R: Renderer;

impl<K, V, R> Clone for Attr<K, V, R>
where
    K: AttributeKey,
    V: AttributeValue<R> + Clone,
    R: Renderer,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<K, V, R> Attribute<R> for Attr<K, V, R>
where
    K: AttributeKey + Send,
    V: AttributeValue<R> + Send,
    R: Renderer,
{
    const MIN_LENGTH: usize = 0;

    type State = V::State;
    type Cloneable = Attr<K, V::Cloneable, R>;
    type CloneableOwned = Attr<K, V::CloneableOwned, R>;

    fn html_len(&self) -> usize {
        K::KEY.len() + 3 + self.1.html_len()
    }

    fn to_html(
        self,
        buf: &mut String,
        _class: &mut String,
        _style: &mut String,
        _inner_html: &mut String,
    ) {
        self.1.to_html(K::KEY, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        self.1.hydrate::<FROM_SERVER>(K::KEY, el)
    }

    fn build(self, el: &R::Element) -> Self::State {
        V::build(self.1, el, K::KEY)
    }

    fn rebuild(self, state: &mut Self::State) {
        V::rebuild(self.1, K::KEY, state);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        Attr(self.0, self.1.into_cloneable(), PhantomData)
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        Attr(self.0, self.1.into_cloneable_owned(), PhantomData)
    }
}

impl<K, V, R> NextAttribute<R> for Attr<K, V, R>
where
    K: AttributeKey,
    V: AttributeValue<R>,
    R: Renderer,
{
    type Output<NewAttr: Attribute<R>> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute<R>>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
    }
}
