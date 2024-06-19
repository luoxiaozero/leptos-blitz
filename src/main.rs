use leptos_blitz::{html::*, prelude::*};

fn main() {
    launch(move || {
        div()
            .child("123")
            .child(br())
            .child(
                div()
                    .style("display: block;")
                    .child(span().style("color:red;").child("child"))
                    .child(button().child("+1")),
            )
            .child(
                a().href("https://github.com/leptos-rs/leptos")
                    .target("_blank")
                    .child("Leptos"),
            )
    })
}
