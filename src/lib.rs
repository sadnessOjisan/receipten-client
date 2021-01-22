// requires the serde and anyhow crates
use wasm_bindgen::prelude::*;
use serde::Deserialize;
use yew::{format::{Json, Nothing}, prelude::*, services::fetch::{FetchService, FetchTask, Request, Response}, web_sys::console::info};

#[derive(Deserialize, Debug, Clone)]
pub struct ISSPosition {
    latitude: String,
    longitude: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    itemName: String,
    itemPrice: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
    data: Vec<Item>
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<ResponseData, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    iss: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
}
/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl FetchServiceExample {
    fn view_iss_location(&self) -> Html {
        match self.iss {
            Some(ref space_station) => {
                html! {
                    <>
                        <p>{ "The ISS is at:" }</p>
                        <p>{format!("Latitude: {:?}", space_station.data )}</p>

                   

                            {for space_station.data.iter().map(|e| self.renderItem(e)) }
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetLocation)>
                         { "Where is the ISS?" }
                     </button>
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }

    fn renderItem(&self, item: &Item) -> Html {
   
        html! {
            <tr>
                                    <td>{ &item.itemName }</td>
                                    <td>{ &item.itemPrice }</td>
                                </tr>
        }
    }
}
impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            iss: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetLocation => {
                // 1. build the request
                let request = Request::get("https://receipten-backend.ojisan.vercel.app/api/get-items?id=JtvoNq7CnSUU6HvB1QPK")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.iss = Some(location);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_iss_location() }
                { self.view_error() }
            </>
        }
    }
}

// wasm module が読み込まれたときのエントリポイント
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    // body タグにSPAをマウント
    App::<FetchServiceExample>::new().mount_to_body();
}


