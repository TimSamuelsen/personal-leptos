use leptos::*;
use leptos_meta::Stylesheet;
use leptos_meta::Link;
use leptos_router::*;

use crate::work::WorkSection;

#[component]
fn Home() -> impl IntoView {
    view! {
        <main class="dark:bg-[#121212] bg-white dark:text-white h-screen w-full py-20 space-y-8 font-robotomono">
            <div class="flex justify-center items-center">
                <div class="text-left space-y-2 pt-5 mb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-[100ch]">
                <p class="text-3xl font-bold">Tim Samuelsen</p>
                <p>MS in Mechanical Engineering graduate student at Stanford University, specializing in mechatronics and 
                smart product design. Software and systems developer for advanced CLIP 3D printers at the DeSimone Research Group.</p>
                </div>
            </div>


            <div class="flex justify-center items-center">
                <div class="pt-5 mb-10 mx-auto px-5 w-full lg:px-0 lg:max-w-[100ch]">
                    <WorkSection/>
                </div>
            </div>
        </main>
        
    }
}

#[component]
pub fn App() -> impl IntoView {

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