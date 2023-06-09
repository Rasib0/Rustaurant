use serde::Deserialize;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use std::{ops::Deref};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;
use crate::utils::struct_to_hex;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub review_exists: String,
    pub hide_fn: Callback<MouseEvent>,
    pub onsubmit: Callback<UserReview>,
    pub initial_user_review: UserReview,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UserReview {
    pub user_rating: i32,
    pub user_review_title: String,
    pub restaurant_name: String,
    pub user_review: String,
    pub user_name: String,
}

#[function_component(WriteAReview)]
pub fn write_a_review(props: &Props) -> Html {
    let my_text_handle_title = use_state(|| props.initial_user_review.user_review_title.clone());
    let my_text_handle_review = use_state(|| props.initial_user_review.user_review.clone());
    let my_text_handle_rating = use_state(|| props.initial_user_review.user_rating.to_string());

    let cloned_title = my_text_handle_title.deref().clone();
    let cloned_review = my_text_handle_review.deref().clone();
    let cloned_rating = my_text_handle_rating.deref().clone();

    let handle_input_title = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Title: {:?}", value).into());
        my_text_handle_title.set(value);
    });

    let handle_input_review = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Review: {:?}", value).into());
        my_text_handle_review.set(value);
    });

    let handle_input_rating = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Rating: {:?}", value).into());
        my_text_handle_rating.set(value);
    });

    let user_review = UserReview {
        user_review_title: cloned_title.clone(),
        user_review: cloned_review.clone(),
        user_rating: cloned_rating.parse::<i32>().unwrap_or(-1),
        restaurant_name: props.initial_user_review.restaurant_name.clone(),
        user_name: "John Doe".to_string(),
    };

    let user_review_hex = struct_to_hex(&user_review);

    let props_rc = Rc::new(RefCell::new(props.clone()));
    let props_clone = props_rc.clone();

    let onsubmit = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        web_sys::console::log_1(&format!("User Review: {:?}", user_review).into());
        props_clone.borrow().hide_fn.emit(e.clone());
    });
    html! {
        <form class="bg-gray-800 shadow-md rounded-lg px-5 py-4 border border-gray-200">
            <div class={props_rc.borrow().review_exists.clone()}>
                <div class="w-2/3 mt-3">
                    <label for="small-input" class="mb- text-2xl font-normal tracking-tight text-primary text-sm">
                        {"Review Title"}
                    </label>
                    <input
                        type="text"
                        placeholder={"Your review title goes here"}
                        value={cloned_title}
                        oninput={handle_input_title}
                        id="small-input"
                        class="block p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 sm:text-xs focus:ring-blue-500 focus:border-blue-500"
                    />
                </div>
                <div>
                    <label for="large-input" class="mb-3 text-2xl font-normal tracking-tight text-primary text-sm">
                        {"My Review"}
                    </label>
                    <input
                        placeholder={"Your review goes here"}
                        value={cloned_review}
                        oninput={handle_input_review}
                        id="large-input"
                        class="block w-full p-4 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 sm:text-md focus:ring-blue-500 focus:border-blue-500"
                    />
                </div>
                <label for="my-dropdown" class="mb-3 text-2xl font-normal tracking-tight text-primary text-sm">
                    {"Select a rating (0-5):"}
                </label>
                <div class="flex flex-row">
                    <input
                        id="my-dropdown"
                        placeholder={"Rate this restaurant 1 to 5"}
                        value={cloned_rating.clone().to_string()}
                        oninput={handle_input_rating}
                        class="block p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 sm:text-xs focus:ring-blue-500 focus:border-blue-500"
                        name="my-dropdown"
                    />
                    <Link<Route> to={Route::Submitting { review_hex: user_review_hex }}>
                    <div>
                        <input
                            onclick={onsubmit}
                            type="submit"
                            value={"Submit"}
                            class="text-white ml-5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center"
                        />
                    </div>
                    </Link<Route>>
                </div>
                <div class={if cloned_rating.parse::<i32>().unwrap_or(-1) < 1 || cloned_rating.parse::<i32>().unwrap_or(-1) > 5 {"block"} else {"hidden"}}>
                    <p class="text-red-500 text-xs italic mt-2">{"Please choose a rating between 1 and 5; otherwise, I will assume it's 5!"}</p>
                </div>
            </div>
            <div class={if props_rc.borrow().review_exists.clone() == "block" {"hidden"} else {"block"}}>
                    <input
                        onclick={props_rc.borrow().hide_fn.clone()}
                        type="submit"
                        value={"Edit Review"}
                        class="text-white ml-5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center"
                    />
            </div>
        </form>
    }
}
