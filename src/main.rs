use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::write_a_review::UserReview;
mod pages;
mod components;

use pages::{
    about::About, home::Home, not_found::NotFound, restaurant::Restaurant, login::Login, create_account::CreateAccount, redirecting::Redirecting, submitting::Submitting
};


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/search/:query")]
    Search { query: String },
    #[at("/redirecting/:route")]
    Redirecting { route: String},
    #[at("/submitting/:route")]
    Submitting { route: String},
    #[at("/about")]
    About,
    #[at("/create_account")]
    CreateAccount,
    #[at("/login")]
    Login,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/restaurant/:name")] //TODO: need to add a dynamic route here 
    Restaurant {name: String}

}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => html! { <About /> },
        Route::Home => html! { <Home query="" /> },
        Route::Search { query } => html! { <Home {query} /> },
        Route::Redirecting {route} => html! { <Redirecting {route} /> },
        Route::Submitting {route} => html! { <Submitting {route} review={
            UserReview {
                user_review_title: "Title".to_string(),
                user_review: "Review".to_string(),
                user_rating: 5,
                user_name: "John Doe".to_string(),
            }
        } /> },
        Route::Secure => html! { <Secure /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Restaurant { name }=> html! { <Restaurant {name} />},
        Route::Login => html! { <Login /> },
        Route::CreateAccount => html! { <CreateAccount /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}


#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

