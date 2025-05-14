// src/pages/home.rs
use gloo::console::log;
use yew::prelude::*;

use crate::components::crypto_list::CryptoList;
use crate::components::error::Error;
use crate::components::loading::Loading;
use crate::models::crypto::PortfolioItem;
use crate::store::{use_store, StoreAction};

#[function_component(Home)]
pub fn home() -> Html {
    let (store, dispatch) = use_store();

    let on_retry = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            dispatch.emit(StoreAction::FetchCryptocurrencies);
        })
    };

    use_effect(move || {
        dispatch.emit(StoreAction::FetchCryptocurrencies);
        || ()
    });

    html! {
        <div class="home-page max-w-3xl mx-auto bg-white rounded-lg shadow p-8 mt-8">
            <h2 class="text-2xl font-bold text-blue-700 mb-6">{"HomePortfolio"}</h2>

            {
                if store.loading {
                    html! { <Loading /> }
                } else if let Some(error) = &store.error {
                    html! { <Error message={error.clone()} on_retry={Some(on_retry)} /> }
                } else if store.cryptocurrencies.is_empty() {
                    html! { <p class="text-gray-500">{"No cryptocurrencies available."}</p> }
                } else {
                    html! { <CryptoList cryptocurrencies={store.cryptocurrencies.clone()} /> }
                }
            }
        </div>
    }
}
