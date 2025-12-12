use futures_util::StreamExt;
use polymarket_rs::request::GammaMarketParams;
use polymarket_rs::types::WsEvent;
use polymarket_rs::websocket::{MarketWsClient, ReconnectConfig, ReconnectingStream};
use polymarket_rs::GammaClient;
use std::time::Duration;

async fn get_market_token_id() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = GammaClient::new("https://gamma-api.polymarket.com");

    let params = GammaMarketParams::new()
        .with_active(true)
        .with_closed(false)
        .with_limit(5);

    match client.get_markets(Some(params)).await {
        Ok(markets) => Ok(markets
            .iter()
            .map(|m| {
                let token_ids: Vec<String> =
                    serde_json::from_str(&m.clob_token_ids.as_ref().unwrap()).unwrap_or_default();
                token_ids
            })
            .flatten()
            .collect()),
        Err(e) => {
            println!("Get markets error: {}", e);
            Err(Box::new(e))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketWsClient::new();

    // Token IDs to subscribe to
    let token_ids = get_market_token_id().await?;

    println!("Connecting to CLOB WebSocket with auto-reconnect...");
    println!("Subscribing to {} token(s)", token_ids.len());

    // Configure reconnection behavior
    let reconnect_config = ReconnectConfig {
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(30),
        multiplier: 2.0,
        max_attempts: None, // Infinite reconnection attempts
    };

    // Create a reconnecting stream that will automatically reconnect on disconnection
    let mut stream = ReconnectingStream::new(reconnect_config, move || {
        let client = client.clone();
        let token_ids = token_ids.clone();
        async move {
            println!("🔄 Connecting to WebSocket...");
            let (stream, _handle) = client.subscribe_with_handle(token_ids).await?;
            println!("✅ Connected successfully!");
            Ok(stream)
        }
    });

    println!("Waiting for events...\n");

    // Process events as they arrive
    let mut event_count = 0;
    while let Some(result) = stream.next().await {
        match result {
            Ok(event) => {
                event_count += 1;
                match event {
                    WsEvent::Book(book) => {
                        println!("[Book Event #{}]", event_count);
                        println!("  Market: {}", book.market);
                        println!("  Asset ID: {}", book.asset_id);
                        println!("  Bids: {} levels", book.bids.len());
                        if let Some(best_bid) = book.bids.first() {
                            println!("    Best bid: {} @ {}", best_bid.size, best_bid.price);
                        }
                        println!("  Asks: {} levels", book.asks.len());
                        if let Some(best_ask) = book.asks.first() {
                            println!("    Best ask: {} @ {}", best_ask.size, best_ask.price);
                        }
                        println!();
                    }
                    WsEvent::PriceChange(change) => {
                        println!("[Price Change Event #{}]", event_count);
                        println!("  Market: {}", change.market);
                        println!("  Changes: {}", change.price_changes.len());
                        for price_change in &change.price_changes {
                            println!(
                                "    {:?} @ {}: {} ({})",
                                price_change.side,
                                price_change.price,
                                price_change.size,
                                if price_change.size.is_zero() {
                                    "removed"
                                } else {
                                    "updated"
                                }
                            );
                        }
                        println!();
                    }
                    WsEvent::LastTradePrice(trade) => {
                        println!("[Trade Event #{}]", event_count);
                        println!("  Market: {}", trade.market);
                        println!("  Asset ID: {}", trade.asset_id);
                        println!("  Trade: {:?} {} @ {}", trade.side, trade.size, trade.price);
                        println!("  Fee: {} bps", trade.fee_rate_bps);
                        println!("  TX: {}", trade.transaction_hash);
                        println!();
                    }
                    WsEvent::TickSizeChange(tick) => {
                        println!("[Tick Size Change Event #{}]", event_count);
                        println!("  Market: {}", tick.market);
                        println!("  New Tick Size: {}", tick.new_tick_size);
                        println!();
                    }
                }
            }
            Err(e) => {
                // With ReconnectingStream, errors are logged but the stream continues
                // Only fatal errors (like max reconnection attempts) will terminate the stream
                eprintln!("⚠️  Error: {} - Reconnecting...", e);
            }
        }
    }

    println!("WebSocket stream ended.");
    Ok(())
}
