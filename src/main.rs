const SIGNIN_URL: &str = "https://signin.aws.amazon.com/federation";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    let creds = config
        .credentials_provider()
        .unwrap()
        .as_ref()
        .provide_credentials()
        .await
        .unwrap();

    let client = reqwest::Client::new();

    let response = client
        .get(SIGNIN_URL)
        .query(&[
            ("Action", "getSigninToken"),
            (
                "Session",
                serde_json::json!({
                    "sessionId": creds.access_key_id(),
                    "sessionKey": creds.secret_access_key(),
                    "sessionToken": creds.session_token().unwrap(),
                })
                .to_string()
                .as_ref(),
            ),
        ])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let parsed: serde_json::Value = serde_json::from_str::<serde_json::Value>(&response).unwrap();
    let token = &parsed["SigninToken"].as_str().unwrap();

    let region = config.region();
    let dest_url = region.map_or_else(
        || "https://console.aws.amazon.com".to_owned(),
        |r| format!("https://{r}.console.aws.amazon.com"),
    );

    let params = serde_urlencoded::to_string([
        ("Action", "login"),
        ("Issuer", "example.com"), // doesn't matter
        ("Destination", &dest_url),
        ("SigninToken", token),
    ])
    .unwrap();

    println!("{SIGNIN_URL}?{params}");
}
