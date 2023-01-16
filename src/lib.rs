use serde_json::json;
use slack_flows::{channel_msg_received, send_message_to_channel};
use store_flows::{del, get, set};

#[no_mangle]
pub fn run() {
    if let Some(sm) = channel_msg_received("reactor-space", "t1") {
        let last_result = match sm.text == "C" {
            true => {
                del("last_result");
                0.0
            }
            false => match get("last_result") {
                Some(v) => v.as_f64().unwrap_or_default(),
                None => 0.0,
            },
        };
        let expr = match sm.text.chars().next() {
            Some(c) => match c {
                '+' | '-' | '*' | '/' => {
                    format!("{}{}", last_result, sm.text)
                }
                _ => sm.text,
            },
            None => sm.text,
        };
        match meval::eval_str(expr) {
            Ok(v) => {
                set("last_result", json!(v));
                send_message_to_channel("reactor-space", "t2", v.to_string());
            }
            Err(_) => {
                send_message_to_channel("reactor-space", "t2", String::from("Invalid expression"));
            }
        }
    }
}
