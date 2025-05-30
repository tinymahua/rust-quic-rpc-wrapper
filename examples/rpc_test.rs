use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use rust_quic_rpc_wrapper::server::start_rpc_server;
use futures_lite::StreamExt;
use rust_quic_rpc_wrapper::client::make_rpc_client;
use rust_quic_rpc_wrapper::proto::{ListArticlesRequest, ShutdownRequest};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {

    let cancellation_token = CancellationToken::new();
    let cancellation_token_cloned = cancellation_token.clone();
    tokio::spawn(async move {
        start_rpc_server(Some(23456), cancellation_token_cloned).await.expect("Failed to start RPC server");
    });

    sleep(Duration::from_secs(5)).await;

    // let cancellation_token_for_client = cancellation_token.clone();
    tokio::spawn(async move {
        let client = make_rpc_client(Some(23456)).await.unwrap();
        let mut articles_stream = client.server_streaming(ListArticlesRequest).await.expect("Failed to get articles stream");
        while let Some(res) = articles_stream.next().await {
            match res {
                Ok(article) => {
                    println!("Received article: {}", article.title);
                },
                Err(err) => {
                    eprintln!("Error receiving article: {}", err);
                }
            }
        }
        sleep(Duration::from_secs(15)).await;
        let res = client.rpc(ShutdownRequest).await.expect("Failed to shutdown");
        println!("Shutdown request received: {:?}", res);
        // cancellation_token_for_client.cancel();
    });



    let main_cancellation_token = cancellation_token.clone();
    loop {
        if main_cancellation_token.is_cancelled() {
            println!("Main thread received cancellation signal");
            break;
        }
    }

    Ok(())
}
