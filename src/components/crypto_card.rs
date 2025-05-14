// src/components/crypto_card.rs
use yew::prelude::*;

use crate::models::crypto;
use crate::models::crypto::Cryptocurrency;
use crate::models::crypto::PortfolioItem;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct CryptoCardProps {
    pub cryptocurrency: Cryptocurrency,
    pub in_portfolio: bool,
    pub amount: Option<f64>,
    pub on_add_to_portfolio: Callback<PortfolioItem>,
    pub on_update_portfolio: Callback<PortfolioItem>,
    pub on_remove_from_portfolio: Callback<String>,
}

#[function_component(CryptoCard)]
pub fn crypto_card(props: &CryptoCardProps) -> Html {
    let amount_input_ref = use_node_ref();
    let amount = use_state(|| props.amount.unwrap_or(0.0));
    let show_amount_input = use_state(|| false);

    let on_amount_change = {
        let amount = amount.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f64>() {
                amount.set(value);
            }
        })
    };

    let on_add_click = {
        let crypto_id = props.cryptocurrency.id.clone();
        let _on_add_to_portfolio = props.on_add_to_portfolio.clone();
        let amount = amount.clone();
        let show_input = show_amount_input.clone();

        Callback::from(move |_| {
            if *show_input {
                _on_add_to_portfolio.emit(PortfolioItem {
                    crypto_id: crypto_id.clone(),
                    amount: *amount,
                });
                show_input.set(false);
            } else {
                show_input.set(true);
            }
        })
    };

    let on_update_click = {
        let crypto_id = props.cryptocurrency.id.clone();
        let _on_update_portfolio = props.on_update_portfolio.clone();
        let amount = *amount;

        Callback::from(move |_| {
            _on_update_portfolio.emit(PortfolioItem {
                crypto_id: crypto_id.clone(),
                amount,
            });
        })
    };

    let on_remove_click = {
        let crypto_id = props.cryptocurrency.id.clone();
        let _on_remove_from_portfolio = props.on_remove_from_portfolio.clone();

        Callback::from(move |_| {
            _on_remove_from_portfolio
                .emit(crypto_id.clone());
        })
    };

    html! {
        <div class="crypto-card bg-white rounded-lg shadow p-6">
            <div class="card-header mb-4">
                <h2 class="text-xl font-bold text-blue-700">{format!("{} ({})", props.cryptocurrency.name, props.cryptocurrency.symbol)}</h2>
            </div>

            <div class="card-body space-y-4">
                <div class="price-info flex items-center space-x-4">
                    <p class="price text-2xl font-bold text-gray-800">{format!("${:.2}", props.cryptocurrency.price)}</p>
                    <p class={format!(
                        "text-sm font-medium {}",
                        if props.cryptocurrency.price_change_24h >= 0.0 { "text-green-600" } else { "text-red-600" }
                    )}>
                        {format!("{:.2}%", props.cryptocurrency.price_change_24h)}
                    </p>
                </div>

                <div class="market-info text-gray-600 space-y-1">
                    <p>{"Market Cap: "}<span class="font-semibold text-gray-800">{format!("${:.2} B", props.cryptocurrency.market_cap / 1_000_000_000.0)}</span></p>
                    <p>{"24h Volume: "}<span class="font-semibold text-gray-800">{format!("${:.2} B", props.cryptocurrency.volume_24h / 1_000_000_000.0)}</span></p>
                </div>

                {if *show_amount_input {
                    html! {
                        <div class="amount-input flex items-center space-x-2">
                            <input
                                type="number"
                                step="0.000001"
                                min="0"
                                placeholder="Enter amount"
                                ref={amount_input_ref}
                                value={amount.to_string()}
                                onchange={on_amount_change}
                                class="border rounded px-2 py-1 w-32 focus:outline-none focus:ring-2 focus:ring-blue-400"
                            />
                            <button
                                class="px-4 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
                                onclick={on_add_click}
                            >
                                {"Add"}
                            </button>
                        </div>
                    }
                } else if props.in_portfolio {
                    html! {
                        <div class="portfolio-actions space-y-2">
                            <div class="amount-display">
                                <p class="text-gray-700">{"Your holdings: "}<span class="font-semibold">{format!("{:.6} {}", *amount, props.cryptocurrency.symbol)}</span></p>
                                <p class="text-gray-700">{"Value: "}<span class="font-semibold text-green-600">{format!("${:.2}", *amount * props.cryptocurrency.price)}</span></p>
                            </div>
                            <div class="portfolio-buttons flex items-center space-x-2">
                                <input
                                    type="number"
                                    step="0.000001"
                                    min="0"
                                    value={amount.to_string()}
                                    onchange={on_amount_change}
                                    class="border rounded px-2 py-1 w-32 focus:outline-none focus:ring-2 focus:ring-blue-400"
                                />
                                <button
                                    class="px-4 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
                                    onclick={on_update_click}
                                >
                                    {"Update"}
                                </button>
                                <button
                                    class="remove px-4 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition"
                                    onclick={on_remove_click}
                                >
                                    {"Remove"}
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="add-button">
                            <button
                                class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
                                onclick={on_add_click}
                            >
                                {"Add to Portfolio"}
                            </button>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
