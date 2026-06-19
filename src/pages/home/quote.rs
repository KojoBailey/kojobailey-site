use leptos::prelude::*;

#[component]
pub fn SectionQuote() -> impl IntoView {
    view! {
        <div id="top-quote" class="top-quote">
            <div class="quote">"Jack of all trades, master of none,"<br/>"but oftentimes better than a master of one."</div>
        </div>
    }
}
