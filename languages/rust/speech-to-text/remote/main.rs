use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct RequestBody {
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "DEEPGRAM_API_KEY";
    let url = "https://api.deepgram.com/v1/listen";

    let client = Client::new();

    let request_body = RequestBody {
        url: "https://dpgr.am/spacewalk.wav".to_string(),
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Token {}", api_key))
        .json(&request_body)
        .send()?;

    let response_text = response.text()?;
    let response_json: Value = serde_json::from_str(&response_text)?;
    let transcript = &response_json["results"]["channels"][0]["alternatives"][0]["transcript"];

    println!("{}", transcript);

    Ok(())
}
