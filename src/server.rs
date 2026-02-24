use futures_util::SinkExt;
use tokio::{net::TcpListener, sync::broadcast};
#[derive(Clone, Debug)]
pub enum Event {
    Start,
    NewUser(String),
}

pub async fn start_server(mut event_rx: broadcast::Receiver<Event>) {
    let listener = TcpListener::bind("127.0.0.1:8881").await.unwrap();

    let (tx, _) = broadcast::channel::<Event>(100);

    let relay_tx = tx.clone();
    tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            let _ = relay_tx.send(event);
        }
    });
    while let Ok((stream, _)) = listener.accept().await {
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            if let Ok(mut ws) = tokio_tungstenite::accept_async(stream).await {
                while let Ok(event) = rx.recv().await {
                    let msg = match event {
                        Event::Start => "{\"type\": \"start\"}".to_owned(),
                        Event::NewUser(user) => {
                            format!("{{\"type\":\"user\", \"nick\":\"{}\"}}", user)
                        }
                    };
                    if ws
                        .send(tokio_tungstenite::tungstenite::Message::Text(msg.into()))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            }
        });
    }
}
