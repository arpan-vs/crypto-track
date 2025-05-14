// src/components/loading.rs
use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading-spinner flex flex-col items-center justify-center py-8">
            <div class="spinner w-12 h-12 border-4 border-blue-400 border-t-transparent rounded-full animate-spin mb-4"></div>
            <p class="text-blue-700 font-semibold">{"Loading..."}</p>
        </div>
    }
}