mod app;
mod home;
mod components;

use leptos::*;

use app::App;

fn main() {
    logging::log!("csr mode - mounting to body");
    mount_to_body(|| view! { <App/> })
}