use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{prelude::*};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open() -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    path: &'a str,
    sum: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let sum = use_state(|| String::new());
    let sum_value = (*sum).clone();

    let path = use_state(|| String::new());

    let resp = use_state(|| String::new());

    let open_file = {
        let path = path.clone();
        let sum = sum.clone();
        log(&*sum);

        Callback::from(move |_| {
            let path = path.clone();
            spawn_local(async move {
                let response = open().await;
                if response.is_null() {
                    log("No file selected");
                } else {
                    path.set(response.as_string().unwrap());
                }
            })
        })
    };

    let checksum = {
        let resp = resp.clone();
        let path = path.clone();
        let sum = sum.clone();
        Callback::from(move |_| {
            let resp = resp.clone();
            let path = path.clone();
            let sum = sum.clone();
            spawn_local(async move {
                if path.is_empty() || sum.is_empty() {
                    return;
                }

                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                let response = invoke(
                    "checksum",
                    to_value(&GreetArgs {
                        path: &*path,
                        sum: &*sum,
                    })
                    .unwrap(),
                )
                .await;
                log(&response.as_string().unwrap());
                resp.set(response.as_string().unwrap());
            });
        })
    };

    let clear = {
        let resp = resp.clone();
        let path = path.clone();
        let sum = sum.clone();

        Callback::from(move |_| {
            resp.set(String::new());
            path.set(String::new());
            sum.set(String::new());
        })
    };

    let handle_sum_input = {
        let sum = sum.clone();
        Callback::from(move |e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            let value = target.unchecked_into::<HtmlInputElement>().value();
            sum.set(value);
        })
    };

    html! {
        <main class="container">
            <div>
            <div>
                <button onclick={open_file}> {"Choose File"} </button>
                <p>{"File path: "}{&*path}</p>
            </div>
            <div>
                <input id="sum-input" placeholder="Enter a sum..." value={sum_value} oninput={handle_sum_input} />
                <div class="row">
                <button onclick={checksum}> {"Checksum"} </button>
                <button onclick={clear}> {"Clear"} </button>
                </div>
            </div>
            <p>{ &*resp }</p>
            </div>
        </main>
    }
}
