use anyhow::Result;
use aws_sdk_lambda::Client;
use cw_macros::{CloudFormationMacroRequest, Response};
use lambda_runtime::{service_fn, Error, LambdaEvent};

static LAYER_ARN_PARAM: &str = "LayerArn";

async fn function_handler(
    event: LambdaEvent<CloudFormationMacroRequest>,
    client: &Client,
) -> Result<Response, Error> {
    let layer_arn = event
        .payload
        .params
        .get(LAYER_ARN_PARAM)
        .unwrap_or_else(|| panic!("Missing {} parameter for the macro", LAYER_ARN_PARAM))
        .as_str()
        .unwrap();

    let mut list_layer_versions_request = client
        .list_layer_versions()
        .layer_name(layer_arn)
        .into_paginator()
        .send();

    let mut max_value = 1;
    while let Some(page) = list_layer_versions_request.next().await {
        for layer_version in page?.layer_versions() {
            if layer_version.version() > max_value {
                max_value = layer_version.version();
            }
        }
    }

    let resp = Response {
        request_id: event.payload.request_id,
        status: "success".to_string(),
        fragment: serde_json::Value::String(format!("{}:{}", layer_arn, max_value)),
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

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_lambda::Client::new(&config);

    let func = service_fn(move |event| {
        let client_ref = client.clone();
        async move { function_handler(event, &client_ref).await }
    });

    lambda_runtime::run(func).await?;

    Ok(())
}
