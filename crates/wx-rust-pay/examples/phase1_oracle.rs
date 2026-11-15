//! Thin JSONL adapter for the pinned WxJava/WxRust differential harness.
//! Run via tools/compat/run_diff.py; all keys/secrets in fixtures are synthetic.
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use serde_json::{Value, json};
use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorParseError};
use wx_rust_pay::util::sign_utils::SignUtils;

type AdapterResult<T> = Result<T, Box<dyn Error>>;

fn execute(fixture: &Value) -> AdapterResult<Value> {
    let input = &fixture["input"];
    match fixture["operation"].as_str().ok_or("operation required")? {
        "common.error" | "common.error.try" => {
            let body = input["body"].as_str().ok_or("body required")?;
            let platform = match input["wx_type"].as_str() {
                None => None,
                Some("MP") => Some(WxType::Mp),
                Some("CP") => Some(WxType::Cp),
                Some("MiniApp") => Some(WxType::MiniApp),
                Some("Open") => Some(WxType::Open),
                Some("Channel") => Some(WxType::Channel),
                Some(_) => return Err("unknown wx_type".into()),
            };
            let error = if fixture["operation"].as_str() == Some("common.error.try") {
                match WxError::try_from_json_with_type(body, platform) {
                    Ok(error) => error,
                    Err(error) => {
                        // Preserve the real Rust type, variant and message.
                        // Explicit exception-category mode compares only categories.
                        return Ok(json!({
                            "kind": "exception",
                            "type": std::any::type_name::<WxErrorParseError>(),
                            "category": error.category(),
                            "message": error.to_string(),
                            "debug": format!("{error:?}")
                        }));
                    }
                }
            } else {
                WxError::from_json_with_type(body, platform)
            };
            Ok(json!({"kind":"value", "value":{
                "error_code":error.error_code,
                "error_msg":error.error_msg,
                "error_msg_en":error.error_msg_en
            }}))
        }
        "pay.sign" => {
            let mut params = HashMap::new();
            for (key, value) in input["params"]
                .as_object()
                .ok_or("params object required")?
            {
                params.insert(
                    key.clone(),
                    value.as_str().ok_or("params must be strings")?.to_owned(),
                );
            }
            let ignored = input["ignored"]
                .as_array()
                .ok_or("ignored array required")?
                .iter()
                .map(|v| v.as_str().ok_or("ignored must contain strings"))
                .collect::<Result<Vec<_>, _>>()?;
            let key = input["sign_key"].as_str().ok_or("sign_key required")?;
            match SignUtils::create_sign(&params, input["sign_type"].as_str(), key, &ignored) {
                Ok(signature) => Ok(json!({"kind":"value","value":signature})),
                Err(error) => Ok(
                    json!({"kind":"exception","type":"WxErrorException","message":error.to_string()}),
                ),
            }
        }
        _ => Ok(json!({"kind":"unsupported","reason":"operation not implemented by this adapter"})),
    }
}

fn main() -> AdapterResult<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or("fixture JSONL path required")?;
    let reader = BufReader::new(File::open(path)?);
    let mut out = BufWriter::new(std::io::stdout().lock());
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let fixture: Value = serde_json::from_str(&line)?;
        let row = json!({"id":fixture["id"],"operation":fixture["operation"],"result":execute(&fixture)?});
        serde_json::to_writer(&mut out, &row)?;
        writeln!(out)?;
    }
    out.flush()?;
    Ok(())
}
