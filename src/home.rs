use crate::{
    blinkies::Blinkybox,
    page::{Category, Page},
};
use maud::{html, Render};
use rocket::response::content::RawHtml;

#[get("/")]
pub fn home_page() -> RawHtml<String> {
    let content = html! {
        p {
            ("hi! i'm zoe,  welcome to my website! ")
            a href="/feed"
            {
                ("an atom feed is available here.")
            }
        }
        iframe
        frameBorder = "0"
        src="https://ring.bicompact.space/zoe-bat/pre"
        title="links from the armisael webring"
        {}
        (Blinkybox)
    };
    let page = Page {
        content,
        category: Category::Home,
        title: "home",
        show_tags: false,
    };
    RawHtml(page.render().into_string())
}