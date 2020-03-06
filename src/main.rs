use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error, debug};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;

#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
struct ApiGatewayInput {
    body: String,
    headers: InHeaders,
    #[serde(rename = "httpMethod")]
    http_method: String,
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
    path: String,
    resource: String
}

#[derive(Serialize)]
struct ApiGatewayOutput {
    #[serde(rename = "statusCode")]
    status_code: i32,
    headers: OutHeaders,
    body: String,
}

#[derive(Serialize)]
struct Body {
    message: String
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct InHeaders {
    #[serde(rename = "X-Slack-Signature")]
    slack_signature: Option<String>
}

#[derive(Serialize)]
struct OutHeaders {
    #[serde(rename = "x-custom-header")]
    x_custom_header: String
}

#[derive(Deserialize)]
struct SlackChallenge {
    token: String,
    challenge: String,
    r#type: String
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Message {
    CustomEvent(CustomEvent),
    SlackChallenge(SlackChallenge),
}


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(input: ApiGatewayInput, c: Context) -> Result<ApiGatewayOutput, HandlerError> {
    debug!("We received: {:?}", input.body);
    match serde_json::from_str::<Message>(&input.body) {
        Ok(message) => Ok(respond(message)),
        Err(err) => {
            error!("Couldn't parse: {}. Got: {}", input.body, err);
            bail!("We fukd");
        }
    }
}

fn respond(m: Message) -> ApiGatewayOutput {
    match m {
        Message::CustomEvent(e) =>  first_name_response(e),
        Message::SlackChallenge(e) => slack_challenge_response(e)
    }
}

fn first_name_response(custom_event: CustomEvent) -> ApiGatewayOutput {
    let out_body = Body{message: format!("Hello, {}. Ready for some, ughhhhhhnfff...., SMASH?", custom_event.first_name)};
    ApiGatewayOutput {
        status_code: 200,
        headers: OutHeaders {
            x_custom_header: "my custom header value".to_string()
        },
        body: serde_json::to_string(&out_body).unwrap(),
    }
}

fn slack_challenge_response(challenge: SlackChallenge) -> ApiGatewayOutput {
    ApiGatewayOutput {
        status_code: 200,
        headers: OutHeaders {
            x_custom_header: "my custom header value".to_string()
        },
        body: challenge.challenge,
    }
}