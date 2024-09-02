use crate::{
    error_template::{AppError, ErrorTemplate},
    sepa::get_structured_refference,
    web_utils::copy_to_clipboard,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/sepa.css"/>

        <Title text="SEPA Utility"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let (structured_refference, set_structured_refference) =
        create_signal(get_structured_refference());
    view! {
        <h1>"SEPA Utility"</h1>
        <section>
            <h2>"Structured Refference Generator"</h2>
            <button on:click=move |_| set_structured_refference.set(get_structured_refference())>"Generate new structured refference"</button>
            <p>{structured_refference}</p>
            <button on:click=move |_| copy_to_clipboard(&structured_refference.get_untracked())>"Copy to clipboard"</button>

            <h3>"Endpoints"</h3>
            <ul>
                <li>
                    <p class="between">
                        <span>"Just the refference"</span>
                        <a target="_blanc" href="/sr/raw">"GET /sr/raw"</a>
                    </p>
                </li>
                <li>
                    <p class="between">
                        <span>"Refference as JSON"</span>
                        <a target="_blanc" href="/sr/json">"GET /sr/json"</a>
                    </p>
                </li>
            </ul>
        </section>
    }
}
