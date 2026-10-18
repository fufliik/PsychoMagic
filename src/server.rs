use mc_server_status::{McClient, ServerData, ServerEdition};
use std::time::Duration;

pub async fn check_online() -> Option<(i64, i64)> {
    
    let address = "188.127.241.231:30775";
    let client = McClient::new().with_timeout(Duration::from_secs(5));

    let status = client.ping(address, ServerEdition::Java).await.ok()?;

    match status.data {
        ServerData::Java(j) => Some((j.players.online, j.players.max)),
        _ => None,
    }
}