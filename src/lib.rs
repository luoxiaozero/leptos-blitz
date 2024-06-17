mod documents;
mod dom;
mod window;

pub use dom::*;

use documents::LeptosDocument;

pub fn launch<F, N>(f: F)
where
    F: FnOnce() -> N + 'static,
    N: IntoView + 'static,
{
    let docment = LeptosDocument::new(f);

    // let window = crate::window::View::new(document);
    // launch_with_window(window)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn main() {
        launch(move || div().child("123"))
    }
}
