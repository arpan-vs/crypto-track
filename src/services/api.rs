
// src/services/api.rs
use reqwasm::http::Request;
use anyhow::Result;
use serde_json::json;

use crate::models::crypto::{Cryptocurrency, PortfolioItem};

// This would normally point to an actual API
const API_BASE_URL: &str = "https://api.example.com";

// Simulated API calls that return dummy data
pub async fn get_cryptocurrencies() -> Result<Vec<Cryptocurrency>> {
    // In a real app, this would be:
    // let response = Request::get(&format!("{}/cryptocurrencies", API_BASE_URL))
    //     .send()
    //     .await?
    //     .json::<Vec<Cryptocurrency>>()
    //     .await?;
    // Ok(response)
    
    // For this example, we'll simulate API data
    let dummy_data = vec![
        Cryptocurrency {
            id: "bitcoin".to_string(),
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            price: 63542.87,
            market_cap: 1245678900000.0,
            volume_24h: 45678900000.0,
            price_change_24h: 2.34,
        },
        Cryptocurrency {
            id: "ethereum".to_string(),
            name: "Ethereum".to_string(),
            symbol: "ETH".to_string(),
            price: 3421.65,
            market_cap: 412345678900.0,
            volume_24h: 21345678900.0,
            price_change_24h: -1.23,
        },
        Cryptocurrency {
            id: "solana".to_string(),
            name: "Solana".to_string(),
            symbol: "SOL".to_string(),
            price: 189.32,
            market_cap: 86234567890.0,
            volume_24h: 7423456789.0,
            price_change_24h: 5.67,
        },
        Cryptocurrency {
            id: "cardano".to_string(),
            name: "Cardano".to_string(),
            symbol: "ADA".to_string(),
            price: 0.93,
            market_cap: 34256789012.0,
            volume_24h: 1923456789.0,
            price_change_24h: -0.42,
        },
        Cryptocurrency {
            id: "polkadot".to_string(),
            name: "Polkadot".to_string(),
            symbol: "DOT".to_string(),
            price: 14.78,
            market_cap: 18234567890.0,
            volume_24h: 987654321.0,
            price_change_24h: 3.18,
        },
    ];
    
    // Simulate network delay
    // gloo::timers::callback::Timeout::new(500, move || {}).forget();
    
    Ok(dummy_data)
}

pub async fn get_cryptocurrency_details(id: &str) -> Result<Cryptocurrency> {
    // In a real app, this would be:
    // let response = Request::get(&format!("{}/cryptocurrencies/{}", API_BASE_URL, id))
    //     .send()
    //     .await?
    //     .json::<Cryptocurrency>()
    //     .await?;
    // Ok(response)
    
    // For this example, we'll simulate API data
    let dummy_data = match id {
        "bitcoin" => Cryptocurrency {
            id: "bitcoin".to_string(),
            name: "Bitcoin".to_string(),
            symbol: "BTC".to_string(),
            price: 63542.87,
            market_cap: 1245678900000.0,
            volume_24h: 45678900000.0,
            price_change_24h: 2.34,
        },
        "ethereum" => Cryptocurrency {
            id: "ethereum".to_string(),
            name: "Ethereum".to_string(),
            symbol: "ETH".to_string(),
            price: 3421.65,
            market_cap: 412345678900.0,
            volume_24h: 21345678900.0,
            price_change_24h: -1.23,
        },
        "solana" => Cryptocurrency {
            id: "solana".to_string(),
            name: "Solana".to_string(),
            symbol: "SOL".to_string(),
            price: 189.32,
            market_cap: 86234567890.0,
            volume_24h: 7423456789.0,
            price_change_24h: 5.67,
        },
        "cardano" => Cryptocurrency {
            id: "cardano".to_string(),
            name: "Cardano".to_string(),
            symbol: "ADA".to_string(),
            price: 0.93,
            market_cap: 34256789012.0,
            volume_24h: 1923456789.0,
            price_change_24h: -0.42,
        },
        "polkadot" => Cryptocurrency {
            id: "polkadot".to_string(),
            name: "Polkadot".to_string(),
            symbol: "DOT".to_string(),
            price: 14.78,
            market_cap: 18234567890.0,
            volume_24h: 987654321.0,
            price_change_24h: 3.18,
        },
        _ => return Err(anyhow::anyhow!("Cryptocurrency not found")),
    };
    
    // Simulate network delay
    // gloo::timers::callback::Timeout::new(700, move || {}).forget();
    
    Ok(dummy_data)
}

pub async fn update_portfolio(portfolio: Vec<PortfolioItem>) -> Result<Vec<PortfolioItem>> {
    // In a real app, this would be:
    // let response = Request::post(&format!("{}/portfolio", API_BASE_URL))
    //     .header("Content-Type", "application/json")
    //     .body(json!(portfolio).to_string())
    //     .send()
    //     .await?
    //     .json::<Vec<PortfolioItem>>()
    //     .await?;
    // Ok(response)
    
    // For this example, we'll just return the same portfolio
    // Simulate network delay
    // gloo::timers::callback::Timeout::new(300, move || {}).forget();
    
    Ok(portfolio)
}
