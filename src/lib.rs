use anyhow;
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use serde::{ Deserialize, Serialize };
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use webhook_flows::{ create_endpoint, request_handler, send_response };

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler(
    _headers: Vec<(String, String)>,
    _subpath: String,
    _qry: HashMap<String, Value>,
    _body: Vec<u8>
) {
    dotenv().ok();
    logger::init();

    match report_as_md(&mut watchers_map, &mut forked_map, &mut starred_map).await {
        Err(_e) => {
            log::error!("Error generating report in md: {:?}", _e);
            send_response(
                400,
                vec![(String::from("content-type"), String::from("text/plain"))],
                "You've entered invalid owner/repo, or the target is private. Please try again."
                    .as_bytes()
                    .to_vec()
            );
            std::process::exit(1);
        }
        Ok(report) => {
            send_response(
                200,
                vec![(String::from("content-type"), String::from("text/plain"))],
                report.as_bytes().to_vec()
            );
        }
    }
}
