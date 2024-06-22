use leptos_blitz::{ev, html, html::*, prelude::*};

fn main() {
    launch(move || {
        let count = RwSignal::new(0);

        html::main().child(
            div()
                .style("background-color: #f1f1f1; padding: 12px 0")
                .on(ev::click, move |_| {
                    count.set(count.get_untracked() + 1);
                })
                .child("Click me.")
                .child(span().style("color:red").child(" Value: ").child(move || count.get()))
            ).child(
                header().child(h1().child("Accessibility")).child(
                    div().child(
                        p().child("Accessibility")
                            .child(" (often abbreviated to")
                            .child(strong().child("A11y"))
                            .child(r#" — as in, "a", then 11 characters, and then "y") in web development means enabling as many people as possible to use websites, even when those people's abilities are limited in some way."#),
                    ).child(p().child("For many people, technology makes things easier. For people with disabilities, technology makes things possible. Accessibility means developing content to be as accessible as possible, no matter an individual's physical and cognitive abilities and how they access the web.")
                    ).child(
                        p()
                            .child(r#"""#)
                            .child(strong().child("The Web is fundamentally designed to work for all people"))
                            .child(r#", whatever their hardware, software, language, location, or ability. When the Web meets this goal, it is accessible to people with a diverse range of hearing, movement, sight, and cognitive ability." ("#)
                            .child(a().href("https://www.w3.org/standards/webdesign/accessibility").target("_blank").child("W3C - Accessibility"))
                            .child(")")
                    ),
                ),
            ).child(
                section()
                    .child(h2().child("Key tutorials"))
            ).child(
                section()
                    .child(h2().child("Other documentation"))
            ).child(
                section()
                    .child(h2().child("See also"))
            ).child(
                div()
                    .style("height: 240px; background-color: #f1f1f1")
                    .child("height: 240px; background-color: #f1f1f1")
            )
    })
}
