mod documents;
mod window;

use documents::{IntoView, LeptosDocument};

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
    use crate::launch;
    use leptos::prelude::*;

    fn main() {
        launch(move || {
            view! {
                <div>
                    "123"
                </div>
            }
        })
    }
}
