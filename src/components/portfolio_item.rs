// src/components/portfolio_item.rs
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::app::Route;
use crate::models::crypto::{self, Cryptocurrency, PortfolioItem as PortfolioItemModel};

#[derive(Properties, PartialEq)]
pub struct PortfolioItemProps {
    pub item: PortfolioItemModel,
    pub cryptocurrency: Option<Cryptocurrency>,
    pub on_update: Callback<PortfolioItemModel>,
    pub on_remove: Callback<String>,
}

#[function_component(PortfolioItem)]
pub fn portfolio_item(props: &PortfolioItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let amount = use_state(|| props.item.amount);
    
    let on_amount_change = {
        let amount = amount.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                amount.set(value);
            }
        })
    };
    
    let on_update_click = {
        let crypto_id = props.item.crypto_id.clone();
        let _on_update = props.on_update.clone();
        let amount = *amount;
        Callback::from(move |_| {
            _on_update.emit(PortfolioItemModel {
                crypto_id: crypto_id.clone(),
                amount,
            });
        })
    };
    
    let on_remove_click = {
        let crypto_id = props.item.crypto_id.clone();
        let _on_remove = props.on_remove.clone();
        Callback::from(move |_| {
            _on_remove.emit(crypto_id.clone());
        })
    };
    
    let on_details_click = {
        let navigator = navigator.clone();
        let id = props.item.crypto_id.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Details { id: id.clone() });
        })
    };
    
    if let Some(crypto) = &props.cryptocurrency {
        let total_value = props.item.amount * crypto.price;
        
        html! {
            <div class="portfolio-item flex items-center justify-between bg-blue-50 rounded p-4 shadow-sm">
                <div class="item-info flex-1 cursor-pointer" onclick={on_details_click.clone()}>
                    <div class="crypto-name flex items-center space-x-2">
                        <span class="symbol font-mono font-bold text-blue-700">{&crypto.symbol}</span>
                        <span class="name text-gray-700">{&crypto.name}</span>
                    </div>
                    <div class="holdings flex items-center space-x-4 mt-2">
                        <span class="amount text-gray-800">{format!("{:.6}", props.item.amount)}</span>
                        <span class="value font-semibold text-green-600">{format!("${:.2}", total_value)}</span>
                    </div>
                </div>
                
                <div class="item-actions flex items-center space-x-2 ml-4">
                    <input
                        type="number"
                        step="0.000001"
                        min="0"
                        value={amount.to_string()}
                        onchange={on_amount_change}
                        class="border rounded px-2 py-1 w-24 focus:outline-none focus:ring-2 focus:ring-blue-400"
                    />
                    <button
                        class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
                        onclick={on_update_click}
                    >
                        {"Update"}
                    </button>
                    <button
                        class="remove px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition"
                        onclick={on_remove_click}
                    >
                        {"Remove"}
                    </button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="portfolio-item loading flex items-center justify-between bg-gray-100 rounded p-4 animate-pulse">
                <div class="item-info">
                    <div class="crypto-name">
                        <span class="symbol text-gray-400">{"Loading..."}</span>
                    </div>
                </div>
            </div>
        }
    }
}
