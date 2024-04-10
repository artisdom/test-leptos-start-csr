use leptos::*;

#[component]
pub fn Page2() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // a "derived signal" is a function that accesses other signals
    // we can use this to create reactive values that depend on the
    // values of one or more other signals
    let double_count = move || count() * 2;

    view! {
        <button
            on:click = move |_| {
                set_count.update(|n| *n += 1);
            }

            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class:red=move || count() % 2 == 1
            class=("button-20", move || count() % 2 == 1)
        >
            "Click me: "

            { move || double_count() }
        </button>

        // NOTE: self-closing tags like <br> need an explicit /
        <br/>

        // We'll update this progress bar every time `count` changes
        <progress
            // static attributes work as in HTML
            max = "50"

            // passing a function to an attribute, reactively sets that attribute
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value = move || count.get()
        ></progress>

        <br/>

        // This progress bar will use `double_count` so it should move twice as fast!
        <progress
            max = "50"
            // derived signals are functions, so they can also reactively update the DOM
            value=double_count
        ></progress>

        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}