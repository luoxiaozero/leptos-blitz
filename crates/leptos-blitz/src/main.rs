use leptos_blitz::prelude::*;

fn main() {
    launch(move || {
        let count = RwSignal::new(0);

        Effect::new(move |_| {
            println!("count: {}", count.get());
        });

        view! {
            <main>
                <div
                    style="background-color: #f1f1f1; padding: 12px 0"
                    on:click=move |_| {
                        count.set(count.get_untracked() + 1);
                    }
                >
                    "Click me."
                    <span style="color:red">
                        " Value: "
                        {move || count.get()}
                    </span>
                </div>
                <header>
                    <h1>
                        "Accessibility"
                    </h1>
                    <div>
                        <p>
                            "Accessibility (often abbreviated to"
                            <strong>
                                "A11y"
                            </strong>
                            r#" — as in, "a", then 11 characters, and then "y") in web development means enabling as many people as possible to use websites, even when those people's abilities are limited in some way."#
                        </p>
                        <p>
                            "For many people, technology makes things easier. For people with disabilities, technology makes things possible. Accessibility means developing content to be as accessible as possible, no matter an individual's physical and cognitive abilities and how they access the web."
                        </p>
                        <p>
                            r#"""#
                            <strong>
                                "The Web is fundamentally designed to work for all people"
                            </strong>
                            r#", whatever their hardware, software, language, location, or ability. When the Web meets this goal, it is accessible to people with a diverse range of hearing, movement, sight, and cognitive ability." ("#
                            <a href="https://www.w3.org/standards/webdesign/accessibility" target="_blank">
                                "W3C - Accessibility"
                            </a>
                            ")"
                        </p>
                    </div>
                </header>
                <section>
                    <h2>"Key tutorials"</h2>
                </section>
                <section>
                    <h2>"Other documentation"</h2>
                </section>
                <section>
                    <h2>"See also"</h2>
                </section>
                <div style="height: 240px; background-color: #f1f1f1">
                    "height: 240px; background-color: #f1f1f1"
                </div>
            </main>
        }
    })
}
