
// src/models/crypto.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cryptocurrency {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
    pub price_change_24h: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortfolioItem {
    pub crypto_id: String,
    pub amount: f64,
}
