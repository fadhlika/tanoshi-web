use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::components::RouterAnchor;

use crate::app::{browse::BrowseRoute, catalogue::CatalogueRoute, AppRoute};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct NavigationBar {}

pub enum Msg {}

impl Component for NavigationBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavigationBar {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
           <div class="block fixed inset-x-0 bottom-0 z-50 bg-white shadow safe-bottom">
           <div id="tabs" class="flex justify-between">
                <RouterAnchor<AppRoute> route=AppRoute::Browse(BrowseRoute::Home) classes="w-full focus:text-tachiyomi-blue hover:text-tachiyomi-blue justify-center inline-block text-center pt-2 pb-1">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"  class="fill-current inline-block mb-1">
                        <path class="heroicon-ui" d="M6.1 21.98a1 1 0 0 1-1.45-1.06l1.03-6.03-4.38-4.26a1 1 0 0 1 .56-1.71l6.05-.88 2.7-5.48a1 1 0 0 1 1.8 0l2.7 5.48 6.06.88a1 1 0 0 1 .55 1.7l-4.38 4.27 1.04 6.03a1 1 0 0 1-1.46 1.06l-5.4-2.85-5.42 2.85zm4.95-4.87a1 1 0 0 1 .93 0l4.08 2.15-.78-4.55a1 1 0 0 1 .29-.88l3.3-3.22-4.56-.67a1 1 0 0 1-.76-.54l-2.04-4.14L9.47 9.4a1 1 0 0 1-.75.54l-4.57.67 3.3 3.22a1 1 0 0 1 .3.88l-.79 4.55 4.09-2.15z"/>
                    </svg>
                    <span class="tab tab-home block text-xs">{"Favorites"}</span>
               </RouterAnchor<AppRoute>>
               <RouterAnchor<AppRoute> route=AppRoute::Browse(BrowseRoute::Catalogue(CatalogueRoute::Select)) classes="w-full focus:text-tachiyomi-blue hover:text-tachiyomi-blue justify-center inline-block text-center pt-2 pb-1">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"  class="fill-current inline-block mb-1">
                        <path class="heroicon-ui" d="M4 3h16a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2zm16 4V5H4v2h16zm0 2H4v10h16V9z"/>
                    </svg>
                    <span class="tab tab-home block text-xs">{"Catalogue"}</span>
               </RouterAnchor<AppRoute>>
               <RouterAnchor<AppRoute> route=AppRoute::Browse(BrowseRoute::Updates) classes="w-full focus:text-tachiyomi-blue hover:text-tachiyomi-blue justify-center inline-block text-center pt-2 pb-1">
                   <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"  class="fill-current inline-block mb-1">
                        <path class="heroicon-ui" d="M15 19a3 3 0 0 1-6 0H4a1 1 0 0 1 0-2h1v-6a7 7 0 0 1 4.02-6.34 3 3 0 0 1 5.96 0A7 7 0 0 1 19 11v6h1a1 1 0 0 1 0 2h-5zm-4 0a1 1 0 0 0 2 0h-2zm0-12.9A5 5 0 0 0 7 11v6h10v-6a5 5 0 0 0-4-4.9V5a1 1 0 0 0-2 0v1.1z"/>
                   </svg>
                   <span class="tab tab-home block text-xs">{"Updates"}</span>
               </RouterAnchor<AppRoute>>
               <RouterAnchor<AppRoute> route=AppRoute::Browse(BrowseRoute::History) classes="w-full focus:text-tachiyomi-blue hover:text-tachiyomi-blue justify-center inline-block text-center pt-2 pb-1">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"  class="fill-current inline-block mb-1">
                            <path class="heroicon-ui" d="M12 22a10 10 0 1 1 0-20 10 10 0 0 1 0 20zm0-2a8 8 0 1 0 0-16 8 8 0 0 0 0 16zm1-8.41l2.54 2.53a1 1 0 0 1-1.42 1.42L11.3 12.7A1 1 0 0 1 11 12V8a1 1 0 0 1 2 0v3.59z"/>
                        </svg>
                        <span class="tab tab-home block text-xs">{"History"}</span>
               </RouterAnchor<AppRoute>>
               <RouterAnchor<AppRoute> route=AppRoute::Browse(BrowseRoute::Settings) classes="w-full focus:text-tachiyomi-blue hover:text-tachiyomi-blue justify-center inline-block text-center pt-2 pb-1">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24"  class="fill-current inline-block mb-1">
                        <path class="heroicon-ui" d="M9 4.58V4c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v.58a8 8 0 0 1 1.92 1.11l.5-.29a2 2 0 0 1 2.74.73l1 1.74a2 2 0 0 1-.73 2.73l-.5.29a8.06 8.06 0 0 1 0 2.22l.5.3a2 2 0 0 1 .73 2.72l-1 1.74a2 2 0 0 1-2.73.73l-.5-.3A8 8 0 0 1 15 19.43V20a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-.58a8 8 0 0 1-1.92-1.11l-.5.29a2 2 0 0 1-2.74-.73l-1-1.74a2 2 0 0 1 .73-2.73l.5-.29a8.06 8.06 0 0 1 0-2.22l-.5-.3a2 2 0 0 1-.73-2.72l1-1.74a2 2 0 0 1 2.73-.73l.5.3A8 8 0 0 1 9 4.57zM7.88 7.64l-.54.51-1.77-1.02-1 1.74 1.76 1.01-.17.73a6.02 6.02 0 0 0 0 2.78l.17.73-1.76 1.01 1 1.74 1.77-1.02.54.51a6 6 0 0 0 2.4 1.4l.72.2V20h2v-2.04l.71-.2a6 6 0 0 0 2.41-1.4l.54-.51 1.77 1.02 1-1.74-1.76-1.01.17-.73a6.02 6.02 0 0 0 0-2.78l-.17-.73 1.76-1.01-1-1.74-1.77 1.02-.54-.51a6 6 0 0 0-2.4-1.4l-.72-.2V4h-2v2.04l-.71.2a6 6 0 0 0-2.41 1.4zM12 16a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0-2a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/>
                    </svg>
                    <span class="tab tab-home block text-xs">{"Settings"}</span>
               </RouterAnchor<AppRoute>>
               </div>
           </div>
        }
    }
}
