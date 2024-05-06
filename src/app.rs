use leptos::*;
use leptos_router::*;

use crate::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        //<h1>"Most basic timtest"</h1>
        <Home/>
        // <Router>
        //     <Routes>
        //         <Route path="" view=  move || view! { <Home/> }/>
        //     </Routes>
        // </Router>
    }
}