use crate::components::footer::Footer;
use maud::{html, Markup, Render, DOCTYPE};
use strum::EnumIter;

#[derive(PartialEq, EnumIter, Copy, Clone)]
pub enum Category {
    Home,
    Blog,
    Links,
}

impl Category {
    pub fn name(&self) -> &'static str {
        match &self {
            Category::Home => "home",
            Category::Blog => "blog",
            Category::Links => "more cyberspace",
        }
    }
    pub fn link(&self) -> &'static str {
        match &self {
            Category::Home => "/",
            Category::Blog => "/blog",
            Category::Links => "/links",
        }
    }
}

pub struct Page {
    pub content: Markup,
    pub category: Category,
    pub path: Vec<&'static str>,
    pub title: &'static str,
}

impl Render for Page {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            meta
            name="viewport"
            content="width=device-width"
            initial-scale="1.0" {}
            meta charset="UTF8" {}
            title {(self.title)}
            link rel="stylesheet" href="/index.css" {}
            html lang=("en") {
                body {
                    div #content {
                        div class="title" {
                            (YouAreHere {path: self.path.clone()})
                            h1 {(self.title)}
                        }
                        (self.content.clone())
                    }
                }
                (Footer {active: self.category})
            }
        }
    }
}

struct YouAreHere {
    path: Vec<&'static str>,
}

impl Render for YouAreHere {
    fn render(&self) -> Markup {
        let mut links: Vec<Markup> = vec![];
        let mut current_link = String::new();
        for (i, link) in self.path.iter().enumerate() {
            current_link.push_str(link);
            let link = if i == 0 { "~/" } else { link };
            links.push(html! {
                a href=(current_link)
                {(link)}
            });
        }
        html! {
            div {
                @for link in links.iter() {
                    (link)
                }
            }
        }
    }
}
