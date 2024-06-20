use leptos_blitz::{ev, html::*, prelude::*};

fn main() {
    launch(move || {
        let count = RwSignal::new(0);

        div()
            .child("123")
            .child(br())
            .child(
                div()
                    .on(ev::click, move |_| {
                        count.set(count.get_untracked() + 12);
                    })
                    .child(span().style("color:red;").child(move || count.get()))
                    .child(
                        button()
                            .on(ev::click, move |_| {
                                println!("click +1");
                            })
                            .child("+1"),
                    ),
            )
            .child(
                a().href("https://github.com/leptos-rs/leptos")
                    .target("_blank")
                    .child("Leptos"),
            )
    })
}
