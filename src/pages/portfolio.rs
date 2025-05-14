use yew::prelude::*;
use std::collections::HashMap;

use crate::store::{use_store, StoreAction};
use crate::components::portfolio_item::PortfolioItem;
use crate::components::loading::Loading;
use crate::components::error::Error;
use crate::models::crypto::PortfolioItem as PortfolioItemModel;

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let (store, dispatch) = use_store();
    
    let on_retry = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| {
            dispatch.emit(StoreAction::FetchCryptocurrencies);
        })
    };
    
    let on_update_item = {
        let dispatch = dispatch.clone();
        Callback::from(move |item: PortfolioItemModel| {
            dispatch.emit(StoreAction::UpdatePortfolioItem(item));
            dispatch.emit(StoreAction::SavePortfolio);
        })
    };
    
    let on_remove_item = {
        let dispatch = dispatch.clone();
        Callback::from(move |id: String| {
            dispatch.emit(StoreAction::RemoveFromPortfolio(id));
            dispatch.emit(StoreAction::SavePortfolio);
        })
    };
    
    // Create a map of cryptocurrency IDs to cryptocurrency objects
    let crypto_map: HashMap<String, _> = store.cryptocurrencies
        .iter()
        .map(|crypto| (crypto.id.clone(), crypto.clone()))
        .collect();
    
    // Calculate total portfolio value
    let total_value = store.calculate_portfolio_value();
    
    // Clone dispatch for use_effect before we use store
    let dispatch_effect = dispatch.clone();
    
    // Check if we need to fetch cryptocurrencies
    let should_fetch = store.cryptocurrencies.is_empty();
    
    // We need to check these values before potentially moving store
    let is_loading = store.loading;
    let error = store.error.clone();
    let is_portfolio_empty = store.portfolio.is_empty();
    
    // Clone necessary parts of the store for rendering instead of moving the whole store
    let portfolio = store.portfolio.clone();
    
    use_effect(move || {
        if should_fetch {
            dispatch_effect.emit(StoreAction::FetchCryptocurrencies);
        }
        || ()
    });
    
    html! {
        <div class="portfolio-page max-w-3xl mx-auto bg-white rounded-lg shadow p-8 mt-8">
            <h2 class="text-2xl font-bold text-blue-700 mb-6">{"My Portfolio"}</h2>
            
            {
                if is_loading && should_fetch {
                    html! { <Loading /> }
                } else if let Some(error_msg) = error {
                    html! { <Error message={error_msg} on_retry={Some(on_retry)} /> }
                } else if is_portfolio_empty {
                    html! {
                        <div class="empty-portfolio text-gray-500 text-center py-8">
                            <p>{"Your portfolio is empty. Add cryptocurrencies from the Home page."}</p>
                        </div>
                    }
                } else {
                    html! {
                        <>
                            <div class="portfolio-summary flex items-center justify-between bg-blue-50 rounded p-4 mb-6">
                                <h3 class="text-lg font-semibold text-blue-700">{"Total Value"}</h3>
                                <p class="total-value text-2xl font-bold text-green-600">{format!("${:.2}", total_value)}</p>
                            </div>
                            
                            <div class="portfolio-list space-y-4">
                                {
                                    portfolio.iter().map(|item| {
                                        let cryptocurrency = crypto_map.get(&item.crypto_id).cloned();
                                        
                                        html! {
                                            <PortfolioItem
                                                key={item.crypto_id.clone()}
                                                item={item.clone()}
                                                cryptocurrency={cryptocurrency}
                                                on_update={on_update_item.clone()}
                                                on_remove={on_remove_item.clone()}
                                            />
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </>
                    }
                }
            }
        </div>
    }
}