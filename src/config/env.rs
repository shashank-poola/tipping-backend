use dotenv::dotenv;
use std::env;

pub fn load_env() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env::var("DATABASE_URL")?;
    env::var("SOLANA_RPC_URL")?;

    Ok(())
}