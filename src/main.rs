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

fn my_handler(e: serde_json::value::Value, c: Context) -> Result<ApiGatewayOutput, HandlerError> {
    debug!("We received: {:?}", e);

    Ok(ApiGatewayOutput {
        status_code: 200,
        headers: Headers {
            x_custom_header: "my custom header value".to_string()
        },
        body: format!("{}", ""),
    })
}

fn serialize_body(body: Body) -> String {
    serde_json::to_string(&body).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialize_body_works() {
        assert_eq!(serialize_body(Body{message: "Ready for some, ughhhhhhnfff...., SMASH?".to_string()}), format!("{{\"message\":\"Ready for some, ughhhhhhnfff...., SMASH?\"}}"));
    }
}