use reqwest::Error;
use serde_json::Value;

async fn shorten(link: String) -> Result<(), Error> {
    let url = "https://api-ssl.bitly.com/v4/shorten";
    let auth =  format!("Bearer {}", std::env::var("ACCESS_TOKEN").expect("Failed to read env variable"));
    let auth = auth.trim();    

    let client = reqwest::Client::new();

    let json_data = format!("{{\"long_url\": \"{}\", \"domain\": \"bit.ly\", \"group_guid\": \"Bn7ljxiyRhn\"}}", link.trim());
    let json_data = json_data.trim();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth)
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());   
    let response_body = response.text().await?;

    let json_response: Value = serde_json::from_str(response_body.as_str()).unwrap();

    println!("Response body:\n{}", json_response["link"]);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Paste the link you would like to shorten");
    let mut link = String::new();

    std::io::stdin()
    .read_line(&mut link)
    .unwrap();

    shorten(link).await?;
    Ok(())
}