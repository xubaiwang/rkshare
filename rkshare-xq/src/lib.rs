use reqwest::Client;

pub mod detail;

pub(crate) fn configured_client() -> anyhow::Result<Client> {
    let client = Client::builder()
        .user_agent(concat!(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            "AppleWebKit/537.36 (KHTML, like Gecko)",
            "Chrome/114.0.0.0",
            "Safari/537.36"
        ))
        .build()?;
    Ok(client)
}
