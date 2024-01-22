use clearkey_bundler::auth::AppAuth;
use clearkey_bundler::{component::*, pages::*};
use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    console_log("From app.rs");

    view! {
      <Router>
        <AppAuth>
            <Navbar />
            <div class="w-full pt-10 overflow-hidden">
              <main class="container min-w-1/2 max-w-screen-lg mx-auto my-auto">
                // all our routes will appear inside <main>
                <Routes>
                  <Route path="/" view=Home />
                </Routes>
              </main>
            </div>
        </AppAuth>
      </Router>
    }
}
