use leptos_blitz::{ev, html::*, prelude::*};

fn main() {
    launch(move || {
        div()
            .child("123")
            .child(br())
            .child(
                div()
                    .child(span().style("color:red;").child("child"))
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
