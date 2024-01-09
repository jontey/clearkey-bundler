use clearkey_bundler::component::navbar::Navbar;
use leptos::{leptos_dom::logging::console_log, *};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    console_log("From app.rs");

    view! {
      <div>
        <Router>
          <Navbar />
          <div class="w-full pt-10 overflow-hidden">
            <main class="container min-w-1/2 max-w-screen-lg mx-auto my-auto">
              // all our routes will appear inside <main>
              <Routes>
                <Route path="/" view=clearkey_bundler::pages::home::Home />
              </Routes>
            </main>
          </div>
        </Router>
      </div>
    }
}
