// src/pages/details.rs
use crate::components::crypto_card::CryptoCard;
use crate::components::error::Error;
use crate::components::loading::Loading;
use crate::models::crypto::PortfolioItem;
use crate::store::{use_store, StoreAction};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DetailsProps {
    pub id: String,
}

#[function_component(Details)]
pub fn details(props: &DetailsProps) -> Html {
    let (store, dispatch) = use_store();

    let on_retry = {
        let dispatch = dispatch.clone();
        let id = props.id.clone();
        Callback::from(move |_| {
            dispatch.emit(StoreAction::FetchCryptocurrencyDetails(id.clone()));
        })
    };

    let on_add_to_portfolio = {
        let dispatch = dispatch.clone();
        Callback::from(move |item: PortfolioItem| {
            dispatch.emit(StoreAction::AddToPortfolio(item));
            dispatch.emit(StoreAction::SavePortfolio);
        })
    };

    let on_update_portfolio = {
        let dispatch = dispatch.clone();
        Callback::from(move |item: PortfolioItem| {
            dispatch.emit(StoreAction::UpdatePortfolioItem(item));
            dispatch.emit(StoreAction::SavePortfolio);
        })
    };

    let on_remove_from_portfolio = {
        let dispatch = dispatch.clone();
        Callback::from(move |id: String| {
            dispatch.emit(StoreAction::RemoveFromPortfolio(id));
            dispatch.emit(StoreAction::SavePortfolio);
        })
    };

    // Check if cryptocurrency is in portfolio
    let portfolio_item = store
        .portfolio
        .iter()
        .find(|item| item.crypto_id == props.id);
    let in_portfolio = portfolio_item.is_some();
    let amount = portfolio_item.map(|item| item.amount);

    let id = props.id.clone();
    use_effect(move || {
        dispatch.emit(StoreAction::FetchCryptocurrencyDetails(id.clone()));
        || ()
    });

    html! {
        <div class="details-page max-w-2xl mx-auto bg-white rounded-lg shadow p-8 mt-8">
            <h2 class="text-2xl font-bold text-blue-700 mb-6">{"Cryptocurrency Details"}</h2>

            {
                if store.loading {
                    html! { <Loading /> }
                } else if let Some(error) = &store.error {
                    html! { <Error message={error.clone()} on_retry={Some(on_retry)} /> }
                } else if let Some(crypto) = &store.selected_cryptocurrency {
                    html! {
                        <CryptoCard
                            cryptocurrency={crypto.clone()}
                            in_portfolio={in_portfolio}
                            amount={amount}
                            on_add_to_portfolio={on_add_to_portfolio}
                            on_update_portfolio={on_update_portfolio}
                            on_remove_from_portfolio={on_remove_from_portfolio}
                        />
                    }
                } else {
                    html! {
                        <p class="text-gray-500">{"No cryptocurrency data available."}</p>
                    }
                }
            }
        </div>
    }
}
