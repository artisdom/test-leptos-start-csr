use leptos::*;
use crate::pages::progress_bar::ProgressBar;

#[component]
pub fn Page5() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    view! {
        <button on:click = move |_| { set_count.update(|n| *n += 1); } >
            "Click me"
        </button>

        <br/>

        <ProgressBar max = 50 progress = count/>

        <ProgressBar progress = count/>

        // Signal::derive creates a Signal wrapper from our derived signal
        // using double_count means it should move twice as fast
        <ProgressBar max = 50 progress = Signal::derive(double_count)/>
    }
}