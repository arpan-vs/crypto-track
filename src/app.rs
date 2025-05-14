use std::env;

use crate::pages::{Details, Home, NotFound, Portfolio};
use crate::store::{Store, StoreAction, StoreContext, StoreProvider, StoreState};
use dotenv::dotenv;
use log;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/details/:id")]
    Details { id: String },
    #[at("/portfolio")]
    Portfolio,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Use COPYRIGHT_TEXT from build-time env, fallback to default if not set
        // dotenv and std::env are not supported in WASM/browser
        let copyright_text = option_env!("COPYRIGHT_TEXT")
            .unwrap_or("© 2023 Crypto Tracker");
        log::info!("Copyright text: {}", copyright_text);

        html! {
            <StoreProvider>
                <BrowserRouter>
                    <div class="app-container min-h-screen flex flex-col bg-gray-50">
                        <header class="bg-white shadow p-4 flex flex-col md:flex-row md:items-center md:justify-between">
                            <h1 class="text-2xl font-bold text-blue-700 mb-2 md:mb-0">{"Crypto Tracker"}</h1>
                            <nav class="flex space-x-4">
                                <Link<Route> to={Route::Home} classes="text-gray-700 hover:text-blue-600 font-medium transition">{"Home"}</Link<Route>>
                                <Link<Route> to={Route::Portfolio} classes="text-gray-700 hover:text-blue-600 font-medium transition">{"Portfolio"}</Link<Route>>
                            </nav>
                        </header>

                        <main class="flex-1 container mx-auto px-4 py-8">
                            <Switch<Route> render={switch} />
                        </main>

                        <footer class="bg-white text-center text-gray-500 py-4 border-t">
                            <p>{ copyright_text }</p>
                        </footer>
                    </div>
                </BrowserRouter>
            </StoreProvider>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Details { id } => html! { <Details id={id} /> },
        Route::Portfolio => html! { <Portfolio /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
