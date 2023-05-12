use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}
#[derive(Deserialize, Debug)]
    struct OAIResponses {
        id: Option<String>,
        object: Option<String>,
        created: Option<u64>,
        model: Option<String>,
        choices: Vec<OAIChoices>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u32,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    
    let oai_token = "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".to_string();
    let auth_header_val = format!("Bearer {}", oai_token);

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read");

        println!("");

        let sp = Spinner::new(&Spinners::Dots9, "\t\tLet me think".into());
        
        let oai_request = OAIRequest {
            prompt: format!("{}", user_text),
            max_tokens: 100,
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);
        let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", &auth_header_val)
        .body(body)
        .unwrap();
        
        let res = client.request(req).await?;

        let body = hyper::body::aggregate(res).await?;

        let json: OAIResponses = serde_json::from_reader(body.reader())?;

        sp.stop();

        println!("");

        println!("{}", json.choices[0].text);

            // Check for Escape key and exit the loop if it is pressed
        if user_text.trim().eq_ignore_ascii_case("exit") {
            break;
        }
    }
    Ok(())
}
