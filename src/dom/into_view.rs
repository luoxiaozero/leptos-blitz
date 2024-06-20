use super::renderer::leptos_dom::LeptosDom;
use leptos::prelude::Render;

pub struct View<T>(T)
where
    T: Sized;

impl<T> View<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

pub trait IntoView
where
    Self: Sized + Render<LeptosDom> + Send,
{
    fn into_view(self) -> View<Self>;
}

impl<T> IntoView for T
where
    T: Sized + Render<LeptosDom> + Send,
{
    fn into_view(self) -> View<Self> {
        View(self)
    }
}

impl<T: IntoView> Render<LeptosDom> for View<T> {
    type State = T::State;

    fn build(self) -> Self::State {
        self.0.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}

// impl<T: IntoView> AddAnyAttr<LeptosDom> for View<T> {
//     type Output<SomeNewAttr: Attribute<LeptosDom>> =
//         <T as AddAnyAttr<LeptosDom>>::Output<SomeNewAttr>;

//     fn add_any_attr<NewAttr: Attribute<LeptosDom>>(self, attr: NewAttr) -> Self::Output<NewAttr>
//     where
//         Self::Output<NewAttr>: RenderHtml<LeptosDom>,
//     {
//         self.0.add_any_attr(attr)
//     }
// }
