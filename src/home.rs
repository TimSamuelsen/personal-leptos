use leptos::*;

use crate::components::work::WorkSection;
use crate::components::postcard::Postcards;

#[component]
pub fn Home() -> impl IntoView {
  let about_me = r#"
  Staff Software Engineer at Carbon, building scalable software systems for automation and device reliability. 
  Previously a graduate student researcher at Stanford, developing software for cutting-edge 3D printing research.
  "#;

  view! {
    <main class="dark:bg-[#121212] bg-white dark:text-white h-screen w-full py-8 space-y-2 font-robotomono">
      <div class="flex justify-center items-center">

        <div class="pt-3 mb-5 mx-auto px-5 w-full lg:px-0 lg:max-w-[100ch]">
          <p class="text-3xl font-bold">Tim Samuelsen</p>

          <div class="grid grid-cols-3 space-y-4">
            <div class="intro-text pt-5">
              <p> 
                {{about_me}}<br/><br/>
              </p>
            </div>

            <div class="sidebar-container"
                 style="min-width: 200px;">
              <SideBar/>
            </div>
          </div>
        </div>
      </div>

      <div class="flex justify-center items-center">
        <div class="pt-5 mb-5 mx-auto px-5 w-full lg:px-0 lg:max-w-[100ch]">
            <WorkSection/>
        </div>
      </div>

      <div class="flex justify-center items-center">
        <div class="pt-5 mb-5 mx-auto px-5 w-full lg:px-0 lg:max-w-[100ch]">
          <Postcards/>
        </div>
      </div>
    </main>      
  }
}

#[component]
fn SideBar() -> impl IntoView {
  view! {
    <div class="sidebar flex-col justify-center items-center space-y-4">
      <div class="sidebar-image">
        <img 
          src="public/tim_headshot_small.png"
          style="border-radius: 50%; border: 2px solid #000000;
                width: 175px; height: 175px;"
        />
      </div>

      <div class="sidebar-buttons w-full flex flex-col justify-center items-center">
        <button
          onclick="window.open('https://www.linkedin.com/in/timsamuelsen', '_blank')"
          class="button-common"
          style="max-width: 200px;"
        >
          <div class="flex items-center">
            <img src="public/linkedin.svg" style="width: 22px; height: 22px; margin-right: 8px;"/>
            <b>LinkedIn</b>
          </div>
          <img src="public/arrow-up-right.svg" style="width: 22px; height: 22px; margin-left: 5px;"/>
        </button>

        <button
          onclick="window.open('https://github.com/TimSamuelsen', '_blank')"
          class="button-common"
          style="max-width: 200px;"
        >
          <div class="flex items-center">
            <img src="public/github.svg" style="width: 22px; height: 22px; margin-right: 8px;"/>
            <b>Github</b>
          </div>
          <img src="public/arrow-up-right.svg" style="width: 22px; height: 22px; margin-left: 5px;"/>
        </button>
      </div>
    </div>
  }
}