// requires the serde and anyhow crates
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
    web_sys::console::info,
};
use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};
use yew_router::{prelude::*, Switch};
mod page;
use page::index::FetchServiceExample;
use yew::virtual_dom::VNode;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/#/item/{id}"]
    FetchServiceExample { id: String },
}

pub enum Msg {
    RouteChanged(Route<()>),
    ChangeRoute(AppRoute),
}

#[derive(Debug)]
pub struct Model {
    route_service: RouteService<()>,
    route: Route<()>,
    link: ComponentLink<Self>,
}
/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl Model {}
impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChanged(route) => self.route = route,
            Msg::ChangeRoute(route) => {
                // This might be derived in the future
                let route_string = match route {
                    AppRoute::FetchServiceExample { id: String } => {
                        format!("/item/")
                    }
                };
                self.route_service.set_route(&route_string, ());
                self.route = Route {
                    route: route_string,
                    state: (),
                };
            }
        }
        true
    }
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();
        let callback = link.callback(Msg::RouteChanged);
        log::info!("{:?}", route);
        route_service.register_callback(callback);
        Model {
            route_service,
            route,
            link,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
            {
                match AppRoute::switch(self.route.clone()) {
                    Some(AppRoute::FetchServiceExample{id}) => html!{<FetchServiceExample id=id />},
                    None => VNode::from("404")
                }
            }
            </>
        }
    }
}

// wasm module が読み込まれたときのエントリポイント
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    // body タグにSPAをマウント
    App::<Model>::new().mount_to_body();
}
