use leptos::*;
use leptos_oidc::{Auth, AuthParameters};

#[component]
pub fn AppAuth(children: Children) -> impl IntoView {
    // Specify OIDC authentication parameters here.
    // Note: This is an example for keycloak, please change it to your needs
    let auth_parameters = AuthParameters {
        auth_endpoint: "https://stnl.us.auth0.com/authorize".to_string(),
        token_endpoint: "https://stnl.us.auth0.com/oauth/token".to_string(),
        client_id: "XxfdVFt38hziznUDY0rtxmDkIUBJxRgZ".to_string(),
        redirect_uri: "http://localhost:8080".to_string(),
        scope: Some("openid profile email offline_access admin:video".to_string()),
        post_logout_redirect_uri: "http://localhost:8080".to_string(),
        logout_endpoint: "https://stnl.us.auth0.com/oidc/logout".to_string(),
    };
    let auth = Auth::init(auth_parameters);

    provide_context(auth);

    view! {
      <div>
        {children()}
      </div>
    }
}
