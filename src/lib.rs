use slack_flows::{channel_msg_received, send_message_to_channel};

#[no_mangle]
pub fn run() {
    if let Some(sm) = channel_msg_received("reactor-space", "t1") {
        send_message_to_channel("reactor-space", "t2", format!("Hello, {}", sm.text));
    }
}
