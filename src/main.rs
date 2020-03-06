extern crate hmac;
extern crate sha2;

use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error, debug};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;
use std::env;

use sha2::Sha256;
use hmac::{Hmac, Mac};

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
    slack_signature: Option<String>,
    #[serde(rename = "X-Slack-Request-Timestamp")]
    slack_timestamp: Option<String>
}

#[derive(Serialize)]
struct OutHeaders {
    #[serde(rename = "x-custom-header")]
    x_custom_header: String
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct SlackChallenge {
    token: String,
    challenge: String,
    r#type: String
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct SlackMention {
    event: SlackEvent
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct SlackEvent {
    r#type: String,
    text: String,
    user: String
}


// TODO: Can be made internally tagged since all slack messages seem to have "type" somewhere
#[derive(Deserialize)]
#[serde(untagged)]
enum InBody {
    CustomEvent(CustomEvent),
    SlackChallenge(SlackChallenge),
    SlackMention(SlackMention)
}


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(input: ApiGatewayInput, c: Context) -> Result<ApiGatewayOutput, HandlerError> {
    debug!("We received: {:?}", input.body);
    match serde_json::from_str::<InBody>(&input.body) {
        Ok(message) => respond(&message, &input.headers, &input.body),
        Err(err) => {
            error!("Couldn't parse: {}. Got: {}", input.body, err);
            bail!("We fukd");
        }
    }
}

fn respond(m: &InBody, headers: &InHeaders, raw_body: &str) -> Result<ApiGatewayOutput, HandlerError> {
    match m {
        InBody::CustomEvent(e) =>  Ok(first_name_response(e)),
        InBody::SlackChallenge(e) => slack_challenge_response(e, &headers, raw_body),
        InBody::SlackMention(e) => slack_mention_response(e, &headers, raw_body)
    }
}

fn first_name_response(custom_event: &CustomEvent) -> ApiGatewayOutput {
    let out_body = Body{message: format!("Hello, {}. Ready for some, ughhhhhhnfff...., SMASH?", custom_event.first_name)};
    ApiGatewayOutput {
        status_code: 200,
        headers: OutHeaders {
            x_custom_header: "my custom header value".to_string()
        },
        body: serde_json::to_string(&out_body).unwrap(),
    }
}

fn slack_challenge_response(challenge: &SlackChallenge, headers: &InHeaders, raw_body: &str) -> Result<ApiGatewayOutput, HandlerError> {
    match slack_verify_signature(raw_body, headers) {
        Err(_) => bail!("Signature did not match"),
        Ok(_) => Ok(ApiGatewayOutput {
            status_code: 200,
            headers: OutHeaders {
                x_custom_header: "my custom header value".to_string()
            },
            body: challenge.challenge.to_owned(),
        })
    }
}

fn slack_mention_response(mention: &SlackMention, headers: &InHeaders, raw_body: &str) -> Result<ApiGatewayOutput, HandlerError> {
    match slack_verify_signature(raw_body, headers) {
        Err(_) => bail!("Signature did not match"),
        Ok(_) => Ok(ApiGatewayOutput {
            status_code: 200,
            headers: OutHeaders {
                x_custom_header: "my custom header value".to_string()
            },
            body: format!("Fuck off, {}. Anyway, have you considered SMASHING with each other? ;)", mention.event.user),
        })
    }
}

fn slack_verify_signature(body: &str, headers: &InHeaders) -> Result<bool, HandlerError> {
    match (&headers.slack_signature, &headers.slack_timestamp) {
        (Some(their_signature), Some(timestamp)) => {
            let our_signature_unhashed = "v0".to_owned() + ":" + &timestamp + ":" + body;
            let signing_secret = env::var("SLACK_SECRET").expect("SLACK_SECRET env variable not found");
            let mut mac = Hmac::<Sha256>::new_varkey(signing_secret.as_bytes())
                .expect("HMAC can take key of any size");
            mac.input(our_signature_unhashed.as_bytes());
            match mac.verify(their_signature.as_bytes()) {
                Err(_) => bail!("Signature did not match"),
                Ok(_) => Ok(true)
            }
        },
        _ => bail!("Didn't have required headers")
    }
}