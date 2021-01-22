use wasm_bindgen::prelude::*;
use yew::{format::{Json, Nothing}, prelude::*, services::{fetch::{self, FetchTask, Request, Response}}};
use yew::services::fetch::{FetchService};

struct Model {
    // リンクコンポーネント -> コンポーネントがコールバックを登録できて自身を更新することができるメカニズム
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    value: i64,
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
}

enum Msg {
    AddOne,  FetchData,
    FetchReady(Result<Item, Error>), // 2.
    Ignore,
}

struct Item {
    itemName: String,
    itemPrice: i32,
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
}


impl Component for Model {

    // Componentトレイトの関連型
    // コンポーネントによって処理され、何らかの副作用を引き起こすことができるさまざまなメッセージを表
    type Message = Msg;

    // Componentトレイトの関連型
    // Propertiesは、親からコンポーネントに渡される情報
    type Properties = ();

    // props と link の初期化に使う. 
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            fetch_service: FetchService::default(),
            fetching: false,
            data: None,
            ft: None,
        }
    }

    // メッセージごとに呼び出されます
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne =>{ self.value += 1}
            Msg::FetchData => {
                self.fetching = true; // 4.

                let callback = self.link.send_message(
                    move |response: Response<Json<Result<Item, Error>>>| { // 2.
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::FetchReady(data)
                        } else {
                            Msg::Ignore
                        }
                    },
                );
                let request = Request::get("/data.json").body(Nothing).unwrap(); // 5.

                let task = FetchService::fetch(request, callback);
            }
            Msg::FetchReady(response) => {
                self.fetching = false; // 4.
                self.data = response.map(|data| data.value).ok(); // 6.
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    // 再レンダリングの管理
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // 実行時にコンポーネントの更新メカニズムにメッセージを送信するコールバックを登録
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
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









// let get_request = Request::get("https://receipten-backend.ojisan.vercel.app/api/get-items?id=JtvoNq7CnSUU6HvB1QPK")
// .body(Nothing)
// .expect("Could not build that request");
// info!("Update: {:?}", get_request);

// let callback = link.callback(|response: Response<Result<String, Error>>| {
// if response.status().is_success() {
//     Msg::Noop
// } else {
//     Msg::Error
// }
// })