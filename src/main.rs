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
    logprobs: Otion<u8>,
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

#[derive(Serialize, Debug)
struct OAIRequest {

fn main () {}
