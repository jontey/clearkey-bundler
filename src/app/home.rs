use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    let Props { title } = props;
    html! {
      <main>
        <div>{format!("This is the {} page", title)}</div>
        <Link<Route> to={Route::Home}>{ "click here to go home" }</Link<Route>>
        <Link<Route> to={Route::Next}>{ "click here to go next" }</Link<Route>>
      </main>
    }
}
