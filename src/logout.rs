use reqwest::Client;

pub async fn logout(client: &Client, address: &String) -> Result<(), Box<dyn std::error::Error>> {
    client.get(address)
        .send().await?;
    Ok(())
}