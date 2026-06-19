mod hero;
mod quote;
mod programming_projects;

use hero::SectionHero;
use quote::SectionQuote;
use programming_projects::SectionProgrammingProjects;

use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="site-header">
                <div class="left">
                    <div class="sitename"><a href="/">"KojoBailey.me"</a></div>
                </div>
                <div class="right">
                    <div class="blog"><a href="https://kojobailey.me" target="_blank">blog</a></div>
                </div>
            </div>
            <SectionHero />
            <SectionQuote />
            <SectionProgrammingProjects />
            <div id="footer" class="site-footer">
                <div class="copyright">"© Kojo Bailey 2026"</div>
            </div>
        </ErrorBoundary>
    }
}
