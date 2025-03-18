use tonic::{Request, transport::Channel};

pub mod proto {
    include!("../rust/proto/mod.rs");
}

use proto::injective::exchange::v1beta1::msg_client::MsgClient;
use proto::injective::exchange::v1beta1::{MsgCreateSpotMarketOrder, OrderInfo, OrderType, SpotOrder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a gRPC client with TLS
    let channel = Channel::from_static("https://testnet.sentry.chain.tm.injective.network:443").await?;

    let mut client = MsgClient::new(channel);

    let spot_order = SpotOrder {
        market_id: "market_id".to_string(),
        order_info: Some(OrderInfo {
            subaccount_id: "subaccount_id".to_string(),
            fee_recipient: "fee_recipient".to_string(),
            price: "price".to_string(),
            quantity: "quantity".to_string(),
            cid: "cid".to_string(),
        }),
        order_type: OrderType::Buy as i32,
        trigger_price: "0".to_string(),
    };
    
    // Create a spot market order
    let order = MsgCreateSpotMarketOrder {
        sender: "sender_address".to_string(),
        order: Some(spot_order),
    };

    // Send the order
    let request = Request::new(order);
    let response = client.create_spot_market_order(request).await?;
    println!("Response: {:?}", response);

    Ok(())
} 