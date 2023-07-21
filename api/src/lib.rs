wit_bindgen::generate!("http-trigger" in "../wit/deps/spin");

use bindings::calculator_eval::{calc, Op};
use exports::fermyon::spin::inbound_http::{self, Request, Response};
use serde::Deserialize;
use anyhow::{anyhow, Result};

struct SpinHttp;
export_http_trigger!(SpinHttp);

impl inbound_http::InboundHttp for SpinHttp {
    fn handle_request(req: Request) -> Response {
        calculate(req).unwrap_or_else(|e| {
            Response {
                status: 500,
                headers: None,
                body: Some(format!("Error: {}", e).into_bytes()),
            }
        })
    }
}

fn calculate(req: fermyon::spin::http_types::Request) -> Result<Response> {
    let query = req.uri.split('?').nth(1).ok_or_else(|| anyhow!( "No query string found"))?;
    let params: QueryParams = serde_qs::from_str(query)?;
    let op = match params.op.as_ref() {
        "add" => Op::Add,
        _ => anyhow::bail!("Unknown operation: {}", params.op)
    };

    let (result, desc) = calc(op, params.x, params.y, "An operation");

    Ok(Response {
        status: 200,
        headers: None,
        body: Some(format!("Result of operation '{}' with values `{}`,`{}`: {result} ({desc})", params.op, params.x, params.y).into_bytes()),
    })
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    x: i32,
    y: i32,
    op: String,
}
