use std::rc::Rc;
use std::collections::HashMap;
use yew::prelude::*;

use crate::models::crypto::{Cryptocurrency, PortfolioItem};
use crate::services::api;

// Define our application state
#[derive(Clone, PartialEq)]
pub struct Store {
    pub cryptocurrencies: Vec<Cryptocurrency>,
    pub portfolio: Vec<PortfolioItem>,
    pub selected_cryptocurrency: Option<Cryptocurrency>,
    pub loading: bool,
    pub error: Option<String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            cryptocurrencies: Vec::new(),
            portfolio: Vec::new(),
            selected_cryptocurrency: None,
            loading: false,
            error: None,
        }
    }
    
    pub fn calculate_portfolio_value(&self) -> f64 {
        let crypto_map: HashMap<String, &Cryptocurrency> = self.cryptocurrencies
            .iter()
            .map(|crypto| (crypto.id.clone(), crypto))
            .collect();
        
        self.portfolio
            .iter()
            .filter_map(|item| {
                crypto_map.get(&item.crypto_id).map(|crypto| crypto.price * item.amount)
            })
            .sum()
    }
}

// Context provider for global state
pub type StoreContext = Rc<UseReducerHandle<StoreState>>;

// Store actions for state management
#[derive(Clone, Debug)]
pub enum StoreAction {
    FetchCryptocurrencies,
    SetCryptocurrencies(Vec<Cryptocurrency>),
    FetchCryptocurrencyDetails(String),
    SetSelectedCryptocurrency(Cryptocurrency),
    AddToPortfolio(PortfolioItem),
    RemoveFromPortfolio(String),
    UpdatePortfolioItem(PortfolioItem),
    SavePortfolio,
    SetError(String),
    ClearError,
    SetLoading(bool),
}

#[derive(PartialEq, Clone)]
pub struct StoreState {
    pub store: Store,
}

impl Reducible for StoreState {
    type Action = StoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_store = match action {
            StoreAction::FetchCryptocurrencies => {
                let mut store = self.store.clone();
                store.loading = true;
                store.error = None;
                store
            },
            StoreAction::SetCryptocurrencies(cryptocurrencies) => {
                let mut store = self.store.clone();
                store.cryptocurrencies = cryptocurrencies;
                store.loading = false;
                store
            },
            StoreAction::FetchCryptocurrencyDetails(_) => {
                let mut store = self.store.clone();
                store.loading = true;
                store.error = None;
                store
            },
            StoreAction::SetSelectedCryptocurrency(cryptocurrency) => {
                let mut store = self.store.clone();
                store.selected_cryptocurrency = Some(cryptocurrency);
                store.loading = false;
                store
            },
            StoreAction::AddToPortfolio(item) => {
                let mut store = self.store.clone();
                if !store.portfolio.iter().any(|i| i.crypto_id == item.crypto_id) {
                    store.portfolio.push(item);
                }
                store
            },
            StoreAction::RemoveFromPortfolio(crypto_id) => {
                let mut store = self.store.clone();
                store.portfolio.retain(|item| item.crypto_id != crypto_id);
                store
            },
            StoreAction::UpdatePortfolioItem(updated_item) => {
                let mut store = self.store.clone();
                if let Some(item) = store.portfolio.iter_mut().find(|i| i.crypto_id == updated_item.crypto_id) {
                    *item = updated_item;
                }
                store
            },
            StoreAction::SavePortfolio => {
                let mut store = self.store.clone();
                store.loading = true;
                store
            },
            StoreAction::SetError(error) => {
                let mut store = self.store.clone();
                store.error = Some(error);
                store.loading = false;
                store
            },
            StoreAction::ClearError => {
                let mut store = self.store.clone();
                store.error = None;
                store
            },
            StoreAction::SetLoading(loading) => {
                let mut store = self.store.clone();
                store.loading = loading;
                store
            },
        };

        Rc::new(Self {
            store: next_store,
        })
    }
}


impl Default for StoreState {
    fn default() -> Self {
        Self {
            store: Store::new(),
        }
    }
}

// Custom hook to use the store
#[hook]
pub fn use_store() -> (Store, Callback<StoreAction>) {
    let store = use_context::<StoreContext>()
        .expect("Store context not set");
    
    let dispatch = {
        let store = store.clone();
        Callback::from(move |action: StoreAction| {
            match &action {
                StoreAction::FetchCryptocurrencies => {
                    let store = store.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match api::get_cryptocurrencies().await {
                            Ok(cryptocurrencies) => {
                                store.dispatch(StoreAction::SetCryptocurrencies(cryptocurrencies));
                            },
                            Err(err) => {
                                store.dispatch(StoreAction::SetError(err.to_string()));
                            }
                        }
                    });
                    // Do NOT dispatch here, async handler will dispatch SetCryptocurrencies/SetError
                    return;
                },
                StoreAction::FetchCryptocurrencyDetails(id) => {
                    let store = store.clone();
                    let id = id.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match api::get_cryptocurrency_details(&id).await {
                            Ok(cryptocurrency) => {
                                store.dispatch(StoreAction::SetSelectedCryptocurrency(cryptocurrency));
                            },
                            Err(err) => {
                                store.dispatch(StoreAction::SetError(err.to_string()));
                            }
                        }
                    });
                    // Do NOT dispatch here, async handler will dispatch SetSelectedCryptocurrency/SetError
                    return;
                },
                StoreAction::SavePortfolio => {
                    let store = store.clone();
                    let portfolio = store.store.portfolio.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match api::update_portfolio(portfolio).await {
                            Ok(_) => {
                                store.dispatch(StoreAction::SetLoading(false));
                            },
                            Err(err) => {
                                store.dispatch(StoreAction::SetError(err.to_string()));
                            }
                        }
                    });
                    // Do NOT dispatch here, async handler will dispatch SetLoading/SetError
                    return;
                },
                _ => {}
            }
            
            store.dispatch(action);
        })
    };
    
    (store.store.clone(), dispatch)
}


#[derive(Properties, Debug, PartialEq)]
pub struct StoreProviderProps {
    #[prop_or_default]
    pub children: Html,
}


#[function_component]
pub fn StoreProvider(props: &StoreProviderProps) -> Html {
    let store = use_reducer(|| StoreState::default());
    let store = Rc::new(store);

    html! {
        <ContextProvider<StoreContext> context={store}>
            {props.children.clone()}
        </ContextProvider<StoreContext>>
    }
}
