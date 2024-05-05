use leptos::*;
use leptos_meta::Stylesheet;
use leptos_meta::Link;
use leptos_router::*;

use crate::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="./style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}