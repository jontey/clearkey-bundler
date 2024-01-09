pub mod home;
// pub mod tauri;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;
use yew::prelude::*;

use home::Home;


#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
  name: &'a str,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/next")]
  Next,
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! { <Home title="Home" /> },
    Route::Next => html! { <Home title="Next" /> }
  }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>
  }
}