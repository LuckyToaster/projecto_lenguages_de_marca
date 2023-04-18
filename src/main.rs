#![allow(dead_code, unused_variables, unused_imports, unused_must_use)]

use yew::prelude::*;
use serde_json::json;
use wasm_bindgen::JsCast; // required for web_sys::HtmlInputElement
use web_sys::HtmlInputElement;
use reqwasm::http::{Request, Response};
use gloo::console::log;

#[derive(Clone)] 
struct Data {
    user_input: String,
    api_output: String,
}

impl Default for Data {
    fn default() -> Self {
        Data { 
            user_input: String::new(),
            api_output: String::from("Submit any text message or email to get an automated reply.")
        }
    }
}

/* 
    state2.set(Data{user_input, ..(*state2).clone()});
    EQUIVALENT TO these 3 lines inside callback:
    let mut state3: Data = (*state2).clone(); 
    state3.api_output = "new output ffs".to_string(); // HTTP REQUEST HERE
    state2.set(state3);
*/ 
#[function_component(App)]
pub fn app() -> Html {
    let state: UseStateHandle<Data> = use_state(|| Data::default());
    let state2 = state.clone();

    let onchange = Callback::from(move |event: Event| {
        let user_input = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
        state2.set(Data{user_input, ..(*state2).clone()});
    });

    let state2 = state.clone();

    let onclick = Callback::from(move |_| {
        state2.set(Data{api_output: get_fake_api_response().unwrap(), ..(*state2).clone()}); // DO API CALL HERE
    });

    html! {
        <>
            <header>
                {"👩‍💼👩‍💻 Welcome to AIssistant. Your digital secretary 👩‍💻👩‍💼"}
            </header>
            <div>
                <textarea type="text"  onchange={onchange} rows=2 placeholder="enter text"/>
                <button type="submit" onclick={onclick}>{"Submit"}</button>
            </div>
            <div>
                <p>{&state.api_output}</p>
            </div>
        </>
    }
}

fn get_api_key() -> Option<String> {
    match std::env::var("OPENAI_API_KEY") {
        Ok(value) => Some(value),
        Err(..) => None,
    }
}

fn get_fake_api_response() -> Option<String> {
    Some("fake ass stringfake ass stringfake ass stringfake ass stringfake ass stringfake ass string
    fake ass stringfake ass stringfake ass stringfake ass stringfake ass string".to_string())
}

fn main() {
    yew::Renderer::<App>::new().render(); 
}


#[allow(dead_code)]
async fn get_gpt_response(msg: String) -> Result<reqwasm::http::Response, reqwasm::Error> {
    let response: Response = Request::get("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", get_api_key().unwrap()).as_str())
        .body(r#"
            '{
                "model": "gpt-3.5-turbo",
                "messages": [{"role": "user", "content": "Say this is a test mofoka!"}],
                "temperature": 0.7
            }'  
        "#).send().await.unwrap();
    Ok(response)
}

#[allow(dead_code)]
async fn get_gpt_response2(msg: String) {
    wasm_bindgen_futures::spawn_local(async move {
        let response: Response = Request::get("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", get_api_key().unwrap()).as_str())
            .body(r#"
                '{
                    "model": "gpt-3.5-turbo",
                    "messages": [{"role": "user", "content": "Say this is a test mofoka!"}],
                    "temperature": 0.7
                }'  
            "#).send().await.unwrap();
        log!(response.status());
    });
}

