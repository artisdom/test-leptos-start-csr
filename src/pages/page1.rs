use leptos::*;

#[component]
pub fn Page1() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click = move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
        </button>

        <p>
            <strong>"Reactive: "</strong>
            // you can insert Rust expressions as values in the DOM by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {move || count()}
        </p>

        <p>
            <strong>"Reactive shorthand: "</strong>
            // signals are functions, so we can remove the wrapping closure
            {count}
        </p>

        <p>
            <strong>"Double reactive: "</strong>
            {move || count() * 2}
        </p>

        <p>
            <strong>"Not reactive: "</strong>
            // NOTE: if you write {count()}, this will *not* be reactive
            // it simply gets the value of count once
            {count() + 1}
        </p>
    }
}
