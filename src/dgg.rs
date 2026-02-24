use std::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use serde_json::Value;

use tokio::{sync::mpsc, time::timeout};
use tokio_tungstenite::connect_async;

const URL: [&str; 2] = ["wss://chat.destiny.gg/ws", "wss://chat.omniliberal.dev/ws"];

pub async fn connect(
    time: u64,
    tx: mpsc::UnboundedSender<String>,
    use_command: bool,
    command: String,
) -> Result<Vec<String>> {
    let mut usernames: Vec<String> = Vec::new();
    let (stream, _) = connect_async(URL[0])
        .await
        .expect("Error connecting to DGG");
    let (_, mut read) = stream.split();
    let result = timeout(Duration::from_secs(time), async {
        while let Ok(msg) = read.next().await.expect("Error Reading Message") {
            if !msg.is_text() {
                continue;
            }
            let msg = msg.into_text().expect("Error Reading Message Text");
            if let Some((prefix, json)) = msg.split_once(" ") {
                if prefix != "MSG" {
                    continue;
                }
                let json: Value = serde_json::from_str(json).expect("Error reading message JSON");

                let nick = json["nick"].as_str().unwrap();
                let data = json["data"].as_str().unwrap();

                if use_command && !data.trim().to_lowercase().starts_with(command.as_str()) {
                    continue;
                }
                if !usernames.contains(&nick.to_owned()) {
                    usernames.push(nick.to_owned());
                    println!("{}", nick);
                    let _ = tx.send(nick.to_owned());
                }
            }
        }
    })
    .await;
    match result {
        Ok(_) => println!(""),
        Err(_) => println!("Time Limit Reached"),
    }
    Ok(usernames)
}
