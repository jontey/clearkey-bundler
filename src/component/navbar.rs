use std::collections::HashMap;

use leptos::{leptos_dom::logging::console_log, *};
use leptos_oidc::{Auth, Authenticated, LoginLink, LogoutLink};
use leptos_router::A;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[non_exhaustive]
struct IdToken {
    name: String,
    value: serde_json::Value,
}

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = expect_context::<Auth>();
    let claims = create_memo(move |_| {
      let id_token = auth.id_token();
      match id_token {
        Some(token) => {
          // return token;
          let json: Result<IdToken, serde_json::Error> = serde_json::from_str(&token.as_str());
          let id_token = json.unwrap_or_default();
          println!("{:?}", id_token);
          console_log(&id_token.value.to_string());
          return id_token.name;
        },
        None => {
          return "".to_string()
        }
      }
      
    });

    let user_name = move || claims.get_untracked();
    console_log(&"navbar");
    console_log(&user_name().to_string());
    // let map: Result<String, serde_json::Value> = serde_json::from_str(&user_name().clone().to_string());
    // println!("hash map = {:#?}", map);

    // let user_name = Signal::derive(move || {
    //     auth.decoded_access_token::<IdToken>()
    //         .get()
    //         .map(|claims| claims.name.clone())
    //         .unwrap_or_default()
    // });
    // let a_token = auth.access_token();

    // if let Some(t) = a_token {
    //     console_log(&t)
    // }

    // if let Some(access_token) = (claims.clone())() {
    //     let signal_wrap = create_memo(move |_| access_token);
    //     let user_name = Signal::derive(move || {
    //       signal_wrap
    //     });
    //     console_log(&serde_json::to_string_pretty(&access_token.unwrap()).unwrap());
    // }

    view! {
      <div class="bg-white w-screen">
        <div class="container mx-auto p-3">
          <nav class="flex flex-col md:flex-row">
            <div class="text-l text-neutral-500 space-x-8 md:flex-none flex">
              <A href="/" class="ml-2">Video Stream Platform</A>
            </div>
            <div class="grow"></div>
            <div class="text-l font-semibold text-indigo-700 flex flex-row space-x-2 justify-end md:flex-row md:space-x-8">
              // {user && <AdminDropdown />}
              <button on:click=move |_| todo!() class="md:ml-2 no-wrap">English</button>
              <button on:click=move |_| todo!() class="md:ml-2 no-wrap">中文</button>
              <button on:click=move |_| todo!() class="md:ml-2 no-wrap">BM</button>
              <Authenticated>
                <div class="md:ml-2 text-neutral-500">{move || user_name}</div>
              </Authenticated>
              <Authenticated
                unauthenticated=move || view! {
                  <LoginLink class="md:ml-2 no-wrap">Login</LoginLink>
                }
              >
                <LogoutLink class="md:ml-2 no-wrap">Logout</LogoutLink>
              </Authenticated>
            </div>
          </nav>
        </div>
      </div>
      <div class="bg-indigo-700 w-screen sm:w-auto shadow-lg">
        <div class="container mx-auto p-3 ">
          <nav class="flex justify-center">
            <div class="text-l text-white space-x-8">
              <A href="/" class="ml-2">Home Page</A>
              {/* <Link href="/" class="ml-2">{t("header_myaccount", "My Account")}</Link> */}
              <A href="/contact-us" class="ml-2">Contact Us</A>
            </div>
          </nav>
        </div>
      </div>
    }
}
