use leptos_blitz::*;

fn main() {
    launch(move || {
        div()
            .child("123")
            .child(br())
            .child({
                div()
                    .child(span().child("child"))
            })
    })
}