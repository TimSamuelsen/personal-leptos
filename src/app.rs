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

                <div class="flex flex-row">
                  <div class="w-2/3">
                    <p>MS in Mechanical Engineering graduate student at Stanford University, specializing in mechatronics and 
                    smart product design. Software and systems developer for advanced CLIP 3D printers at the DeSimone Research Group.</p>
                  </div>

                  <div class="w-1/3 flex flex-col justify-center items-center space-y-4">
                    <SideBar/>
                  </div>
                </div>
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
fn SideBar() -> impl IntoView {
  view! {
    <img 
      src="/public/work_bag.svg" 
      style="width: 100px;
            height: 100px;
            margin-left: 15px; /* Space between icon and text */"
      />
      <button
          onclick="window.open('https://www.linkedin.com/in/timsamuelsen', '_blank')"
          class="button-common"
          style="width: 80%;"
      >
      <div class="flex items-center">
        <img src="/public/linkedin2.svg" style="width: 22px; height: 22px; margin-right: 8px;"/>
        <b>LinkedIn</b>
      </div>
        <img src="/public/arrow-up-right.svg" style="width: 22px; height: 22px; margin-left: 5px;"/>
      </button>
        <button
            onclick="window.open('https://github.com/TimSamuelsen', '_blank')"
            class="button-common"
            style="width: 80%;"
        >
        <div class="flex items-center">
          <img src="/public/github.svg" style="width: 22px; height: 22px; margin-right: 8px;"/>
          <b>Github</b>
        </div>
          <img src="/public/arrow-up-right.svg" style="width: 22px; height: 22px; margin-left: 5px;"/>
        </button>
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