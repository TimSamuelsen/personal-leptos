
use leptos::*;

pub fn Projects() -> impl IntoView {
  let experiences = vec![
        PostMetadata {
          image_path: String::from("/public/carbon_backpack.jpg"),
          title: String::from("backypacky"),
          project_link: String::from("https://www.carbon3d.com/products/ao-suite"),
        },
        PostMetadata {
          image_path: String::from("/public/science_cover_iclip.jpg"),
          title: String::from("me"),
          project_link: String::from("https://www.science.org/doi/10.1126/sciadv.abq3917")
        },
    ];

  view! {
    <div class="grid sm:grid-cols-2 lg:grid-cols-2 gap-8">
    {experiences.into_iter().map(|post_metadata| (PostCardProps { post_metadata: post_metadata })).collect::<Vec<_>>()}
    </div>
  }

}

#[derive(Clone)]
pub struct PostMetadata {
  pub image_path: String,
  pub title: String,
  pub project_link: String,
}

#[component]
pub fn PostCard(post_metadata: PostMetadata) -> impl IntoView {
    view! {
        <a
          class="group flex flex-col transition-all duration-300 p-5 hover:shadow-black/[.4] grayscale hover:grayscale-0"
          href=post_metadata.project_link
          target="_blank"
          rel="noopener noreferrer"
        >
          <div class="aspect-w-11 aspect-h-11">
              <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
          </div>
          <div class="my-1">
              <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                  {post_metadata.title}
              </h3>
          </div>
        </a>
    }
}