use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::page1::Page1;
use crate::pages::page2::Page2;
use crate::pages::page3::Page3;
use crate::pages::page4::Page4;
use crate::pages::page5::Page5;
use crate::pages::page6::Page6;
use crate::pages::page7::Page7;
use crate::pages::page8::Page8;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/page1" view=Page1/>
                <Route path="/page2" view=Page2/>
                <Route path="/page3" view=Page3/>
                <Route path="/page4" view=Page4/>
                <Route path="/page5" view=Page5/>
                <Route path="/page6" view=Page6/>
                <Route path="/page7" view=Page7/>
                <Route path="/page8" view=Page8/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}
