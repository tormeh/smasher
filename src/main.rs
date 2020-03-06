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
    headers: Headers,
    body: String,
}

#[derive(Serialize)]
struct Body {
    message: String
}

#[derive(Serialize)]
struct Headers {
    #[serde(rename = "x-custom-header")]
    x_custom_header: String
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(input: ApiGatewayInput, c: Context) -> Result<ApiGatewayOutput, HandlerError> {
    debug!("We received: {:?}", input.body);
    match serde_json::from_str::<CustomEvent>(&input.body) {
        Ok(custom_event) => {
            let out_body = Body{message: format!("Hello, {}. Ready for some, ughhhhhhnfff...., SMASH?", custom_event.first_name)};

            Ok(ApiGatewayOutput {
                status_code: 200,
                headers: Headers {
                    x_custom_header: "my custom header value".to_string()
                },
                body: serde_json::to_string(&out_body).unwrap(),
            })
        },
        Err(err) => {
            error!("Couldn't parse: {}. Got: {}", input.body, err);
            bail!("We fukd");
        }
    }
}
