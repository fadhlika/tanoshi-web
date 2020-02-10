use yew::{Callback, ClickEvent, Component, ComponentLink, html, Html, Properties, ShouldRender};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub thumbnail: String,
}

pub struct TopBar {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click
}

impl Component for TopBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TopBar {
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="pure-menu pure-menu-horizontal">
                <a href="/" class="pure-menu-heading pure-menu-link">{"Tanoshi"}</a>
                    <ul class="pure-menu-list">
                        <li class="pure-menu-item"><a href="/updates" class="pure-menu-link">{"Updates"}</a></li>
                        <li class="pure-menu-item pure-menu-has-children pure-menu-allow-hover">
                            <a href="#" class="pure-menu-link">{"Catalogue"}</a>
                            <ul class="pure-menu-children">
                                <li class="pure-menu-item"><a href="/catalogue/mangasee" class="pure-menu-link">{"Mangasee"}</a></li>
                                <li class="pure-menu-item"><a href="/catalogue/mangadex" class="pure-menu-link">{"Mangadex"}</a></li>
                                <li class="pure-menu-item"><a href="/catalogue/mangaplus" class="pure-menu-link">{"Mangaplus"}</a></li>
                            </ul>
                        </li>
                        <li class="pure-menu-item"><a href="/settings" class="pure-menu-link">{"Settings"}</a></li>
                    </ul>
            </div>
        }
    }
}