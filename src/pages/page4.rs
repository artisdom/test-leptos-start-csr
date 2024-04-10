use leptos::*;

#[component]
pub fn Page4() -> impl IntoView {
    let html = "<p>This HTML will be injected.</p>";

    view! {
        <div inner_html=html/>
    }
}