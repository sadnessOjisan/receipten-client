// requires the serde and anyhow crates
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
    web_sys::console::info,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    itemName: String,
    itemPrice: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
    data: Vec<Item>,
    sum: i32
}

#[derive(Debug)]
pub enum Msg {
    SuccessFetchData(Result<ResponseData, anyhow::Error>),
}

#[derive(Debug)]
pub struct Receipt {
    fetch_task: Option<FetchTask>,
    data: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
    id: String,
}

impl Receipt {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                html! {
                    <>
                            {for res.data.iter().map(|e| self.renderItem(e)) }
                            <p class="sum">{"小計: "}{res.sum}<span>{"円"}</span></p>
                    </>
                }
            }
            None => {
                html! {
                     <></>
                }
            }
        }
    }
    fn fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }

    fn renderItem(&self, item: &Item) -> Html {
        html! {
            <a class="item" href=format!("https://www.mercari.com/jp/search/?keyword={}", &item.itemName) target="_blank">
                  <div class="left">{ &item.itemName }</div>
                   <div class="right">{ &item.itemPrice }</div>
            </a>
        }
    }
}
impl Component for Receipt {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get(format!(
            "https://receipten-backend.ojisan.vercel.app/api/get-items?id={}",
            _props.id
        ))
        .body(Nothing)
        .expect("Could not build request.");
        // 2. construct a callback
        let callback = link.callback(
            |response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::SuccessFetchData(data)
            },
        );
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        Self {
            fetch_task: Some(task),
            data: None,
            link,
            error: None,
            id: _props.id,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            SuccessFetchData(response) => {
                match response {
                    Ok(data) => {
                        self.data = Some(data);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <div class="container">
            <h1><span>{"おのうえ商店"}</span></h1>
                { self.fetching() }
                { self.success() }
                { self.error() }
          <div class="bottom">
          <a href={format!("https://twitter.com/intent/tweet?text=こんなに長いレシートを作っちゃった！ https://receipten.web.app/{}/item/{}", "%23", self.id)} target="_blank">
          <button>{"Twitter でシェアする"}</button>
         </a>
          </div>
            </div>
        }
    }
}
