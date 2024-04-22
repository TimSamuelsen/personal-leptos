mod work;

use leptos::*;
use leptos_meta::Stylesheet;

use work::WorkSection;

#[component]
fn App() -> impl IntoView {
    //let (count, set_count) = create_signal(0);

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>

        <h2 class="text-2xl font-bold text-black">
            "A start"
        </h2>

        <div class="h-90-screen flex justify-center items-center">
            <div class="pt-20 mb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-[90ch]">
                <WorkSection/>
            </div>
        </div>
        
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}