mod work;

use leptos::*;
use leptos_meta::Stylesheet;
use leptos_meta::Link;
use leptos_router::*;

use work::WorkSection;

#[component]
fn Home() -> impl IntoView {
    view! {
        <main class="dark:bg-[#121212] bg-white dark:text-white h-screen w-full py-20 space-y-8 font-robotomono">
            
            <div class="flex justify-center items-center]">
                <p class="text-3xl">Tim Samuelsen</p>
                <p>Set up a client side rendered Leptos app with one command</p>
            </div>
            <div class="flex justify-center items-center">
                <div class="pt-5 mb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-[90ch]">
                    <WorkSection/>
                </div>
            </div>
        </main>
        
    }
}

#[component]
fn App() -> impl IntoView {

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

fn main() {
    // _ = console_log::init_with_level(log::Level::Debug);
    // console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");
    mount_to_body(|| view! { <App/> })
}