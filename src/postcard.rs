
use leptos::*;

pub fn Projects() -> impl IntoView {
  view! {
    <div class="grid sm:grid-cols-2 lg:grid-cols-2 gap-8">
                <PostCard
                  post_metadata= PostMetadata {
                    image_path: "/public/stanford.svg".to_string(),
                    title: "me".to_string(),
                    date: "01/01/01".to_string(),
                    description: "test".to_string(),
                    project_link: "www.google.com".to_string(),
                  }
                  path="test".to_string()
                />
                <PostCard
                  post_metadata= PostMetadata {
                    image_path: "/public/stanford.svg".to_string(),
                    title: "Patent".to_string(),
                    date: "01/01/01".to_string(),
                    description: "test".to_string(),
                    project_link: "www.google.com".to_string(),
                  }
                  path="test".to_string()
                />
    </div>
  }

}

#[derive(Clone)]
pub struct PostMetadata {
  pub image_path: String,
  pub title: String,
  pub date: String,
  pub description: String,
  pub project_link: String,
}

#[component]
pub fn PostCard(post_metadata: PostMetadata, path: String) -> impl IntoView {
    view! {
        <a
            class="group flex flex-col h-full border transition-all duration-300 rounded-xl p-5 border-gray-700 hover:border-transparent hover:shadow-black/[.4]"
            href=format!("/{}/{}", path, "post_metadata".to_string())
        >
            <div class="aspect-w-16 aspect-h-11">
                <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
            </div>
            <div class="my-6">
                <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                    {post_metadata.title}
                </h3>
                <h2 class="mt-5 text-gray-400">{post_metadata.date}</h2>
                <p class="mt-5 text-[#CED4DA]">{post_metadata.description}</p>
            </div>
            <div class="mt-auto flex items-center gap-x-3">
                <img class="w-8 h-8 rounded-full" src="https://github.com/itehax.png"/>
                <h5 class="text-sm text-gray-200">"By Itehax."</h5>
            </div>
        </a>
    }
}