// src/components/crypto_list.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::models::crypto::Cryptocurrency;

#[derive(Properties, PartialEq)]
pub struct CryptoListProps {
    pub cryptocurrencies: Vec<Cryptocurrency>,
}

#[function_component(CryptoList)]
pub fn crypto_list(props: &CryptoListProps) -> Html {
    let navigator = use_navigator().unwrap();

    let on_crypto_click = {
        let navigator = navigator.clone();
        Callback::from(move |id: String| {
            navigator.push(&Route::Details { id });
        })
    };

    html! {
        <div class="crypto-list divide-y divide-gray-200">
            {props.cryptocurrencies.iter().map(|crypto| {
                let id = crypto.id.clone();
                let on_click = {
                    let id = id.clone();
                    let on_crypto_click = on_crypto_click.clone();
                    Callback::from(move |_| {
                        on_crypto_click.emit(id.clone());
                    })
                };
                
                html! {
                    <div
                        key={id.clone()}
                        class="crypto-item flex items-center justify-between py-4 px-2 hover:bg-blue-50 cursor-pointer transition"
                        onclick={on_click}
                    >
                        <div class="crypto-name flex items-center space-x-2">
                            <span class="symbol font-mono font-bold text-blue-700">{&crypto.symbol}</span>
                            <span class="name text-gray-700">{&crypto.name}</span>
                        </div>
                        <div class="crypto-price flex items-center space-x-4">
                            <span class="price font-semibold text-gray-800">{format!("${:.2}", crypto.price)}</span>
                            <span class={format!(
                                "ml-2 text-sm font-medium {}",
                                if crypto.price_change_24h >= 0.0 { "text-green-600" } else { "text-red-600" }
                            )}>
                                {format!("{:.2}%", crypto.price_change_24h)}
                            </span>
                        </div>
                    </div>
                }
            }).collect::<Html>()}
        </div>
    }
}
