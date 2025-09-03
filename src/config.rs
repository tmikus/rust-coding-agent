use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().context("Failed to load .env file")?;
        Ok(Self {
            api_key: env::var("ANTHROPIC_API_KEY").context("Missing ANTHROPIC_API_KEY")?.to_string(),
        })
    }
}
