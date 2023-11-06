use anyhow::Result;
use cw_macros::{CloudFormationMacroRequest, Response};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

static INPUT_STRING_PARAM: &str = "InputString";
static LENGTH_PARAM: &str = "Length";

async fn function_handler(
    event: LambdaEvent<CloudFormationMacroRequest>,
) -> Result<Response, Error> {
    let content = event
        .payload
        .params
        .get(INPUT_STRING_PARAM)
        .unwrap_or_else(|| panic!("Missing {} parameter for the macro", INPUT_STRING_PARAM))
        .as_str()
        .unwrap();
    let length = event
        .payload
        .params
        .get(INPUT_STRING_PARAM)
        .unwrap_or_else(|| panic!("Missing {} parameter for the macro", LENGTH_PARAM))
        .as_u64()
        .unwrap() as usize;

        let result = if content.len() > length {
            &content[0..length]
        } else {
            content
        };
    
    let resp = Response {
        request_id: event.payload.request_id,
        status: "success".to_string(),
        fragment: serde_json::Value::String(result.to_string()),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
