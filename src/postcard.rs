
use leptos::*;

pub fn Projects() -> impl IntoView {
  let experiences = vec![
        PostMetadata {
          image_path: String::from("/public/carbon_backpack_crop.jpg"),
          title: String::from("backypacky"),
          project_link: String::from("https://www.carbon3d.com/products/ao-suite-hardware"),
        },
        PostMetadata {
          image_path: "/public/sciene_cover_iclip.jpg".to_string(),
          title: "me".to_string(),
          project_link: "https://www.science.org/doi/10.1126/sciadv.abq3917".to_string(),
        },
        PostMetadata {
          image_path: "/public/github.svg".to_string(),
          title: "Patent".to_string(),
          project_link: "www.google.com".to_string(),
        }
    ];

  view! {
    <div class="grid sm:grid-cols-3 lg:grid-cols-3 gap-6">
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
          class="group flex flex-col h-full border transition-all duration-300 p-5 border-gray-700 hover:shadow-black/[.4] grayscale hover:grayscale-0"
          href=post_metadata.project_link
        >
          <div class="aspect-w-11 aspect-h-11">
              <img class="w-full object-cover rounded-xl" src=post_metadata.image_path/>
          </div>
          <div class="my-6">
              <h3 class="text-xl font-semibold  text-gray-300 group-hover:text-[#F8F9FA]">
                  {post_metadata.title}
              </h3>
          </div>
        </a>
    }
}