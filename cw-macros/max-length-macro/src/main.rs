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
        .get(LENGTH_PARAM)
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use serde_json::json;

    // Mocks a LambdaEvent with given input string and length parameters.
    fn mock_event(input: &str, length: u64) -> LambdaEvent<CloudFormationMacroRequest> {
        LambdaEvent {
            payload: CloudFormationMacroRequest {
                account_id: "123456789012".to_string(),
                fragment: HashMap::new(),
                transform_id: "testTransform".to_string(),
                request_id: "testRequest".to_string(),
                region: "us-east-1".to_string(),
                params: {
                    let mut h = HashMap::new();
                    h.insert(INPUT_STRING_PARAM.to_string(), json!(input));
                    h.insert(LENGTH_PARAM.to_string(), json!(length));
                    h
                },
                template_parameter_values: HashMap::new(),
            },
            context: lambda_runtime::Context::default(),
        }
    }

    #[tokio::test]
    async fn test_length_bigger_than_string() {
        let input_string = "Hello";
        let length = 10; // Bigger than the input string length
        let event = mock_event(input_string, length);

        let response = function_handler(event).await.unwrap();

        assert_eq!(response.fragment, json!(input_string));
    }

    #[tokio::test]
    async fn test_length_smaller_than_string() {
        let input_string = "Hello, World!";
        let length = 5; // Smaller than the input string length
        let event = mock_event(input_string, length);

        let response = function_handler(event).await.unwrap();

        assert_eq!(response.fragment, json!("Hello"));
    }
}
