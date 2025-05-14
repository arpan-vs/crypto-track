// src/main.rs
use yew::prelude::*;
use yew_router::prelude::*;

mod app;
mod components;
mod models;
mod pages;
mod services;
mod store;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
