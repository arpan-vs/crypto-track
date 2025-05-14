// src/pages/not_found.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();
    
    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };
    
    html! {
        <div class="not-found-page flex flex-col items-center justify-center min-h-[60vh] text-center space-y-4">
            <h2 class="text-3xl font-bold text-red-600">{"404 - Page Not Found"}</h2>
            <p class="text-lg text-gray-600">{"The page you are looking for does not exist."}</p>
            <button
                class="mt-4 px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
                onclick={go_home}
            >
                {"Go to Home"}
            </button>
        </div>
    }
}
