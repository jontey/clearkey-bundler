use leptos::*;
use leptos_router::A;

#[component]
pub fn Navbar() -> impl IntoView {
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
              // {user && <div class="md:ml-2 text-neutral-500">{user.name}</div>}
              // {user ?
              //   <Link href="/api/auth/logout" class="md:ml-2 no-wrap">{t("header_logout", "Logout")}</Link> :
              //   <Link href="/api/auth/login" class="md:ml-2 no-wrap">{t("header_login", "Login")}</Link>
              // }
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