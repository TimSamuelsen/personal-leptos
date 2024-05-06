use leptos::*;

use crate::components::work::WorkSection;
use crate::components::postcard::MyPostcards;

#[component]
pub fn Home() -> impl IntoView {
  let about_me_p1 = r#"
      MS in Mechanical Engineering graduate student at Stanford University, specializing in mechatronics and 
      smart product design. Software and systems developer for advanced CLIP 3D printers at the DeSimone Research Group.
  "#;
  let about_me_p2 = r#"
    Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been 
    the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley 
    of type and scrambled it to make a type specimen book.
  "#;

  view! {
    <main class="dark:bg-[#121212] bg-white dark:text-white h-screen w-full py-20 space-y-2 font-robotomono">
      <div class="flex justify-center items-center">
        <div class="pt-5 mb-5 mx-auto px-5 w-full lg:px-0 lg:max-w-[100ch]">
          <p class="text-3xl font-bold">Tim Samuelsen</p>

          <div class="flex flex-row">
            <div class="w-2/3 pt-5">
              <p> 
                {{about_me_p2}}<br/><br/>
                {{about_me_p2}}
              </p>
            </div>

            <div class="w-1/3 flex flex-col justify-center items-center space-y-4"
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
          <MyPostcards/>
        </div>
      </div>
    </main>      
  }
}

#[component]
fn SideBar() -> impl IntoView {
  view! {
    <img 
      src="public/tim_headshot_small.png"
      style="border-radius: 50%; border: 2px solid #000000;
             width: 175px; height: 175px;"
    />
    <button
      onclick="window.open('https://www.linkedin.com/in/timsamuelsen', '_blank')"
      class="button-common"
      style="width: 80%;"
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
      style="width: 80%;"
    >
      <div class="flex items-center">
        <img src="public/github.svg" style="width: 22px; height: 22px; margin-right: 8px;"/>
        <b>Github</b>
      </div>
      <img src="public/arrow-up-right.svg" style="width: 22px; height: 22px; margin-left: 5px;"/>
    </button>
  }
}