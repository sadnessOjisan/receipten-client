use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    // リンクコンポーネント -> コンポーネントがコールバックを登録できて自身を更新することができるメカニズム
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
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
        }
    }

    // メッセージごとに呼び出されます
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
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
    // body タグにSPAをマウント
    App::<Model>::new().mount_to_body();
}