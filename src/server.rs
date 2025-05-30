use std::net::SocketAddr;
use std::time::Duration;
use quic_rpc::RpcServer;
use quic_rpc::transport::quinn::{make_server_endpoint, QuinnListener};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use crate::proto::{Request, Response, RpcService};
use crate::rpc::{Handler};

pub async  fn check_cancellation_token(token: CancellationToken) -> bool {
    token.is_cancelled()
}

pub async fn check_ctrl_c() -> bool {
    match tokio::signal::ctrl_c().await {
        Ok(_) => {
            println!("catch ctrl-c");
            true
        },
        Err(_) => {
            false
        }
    }
}

pub async fn start_rpc_server(port: Option<usize>, cancellation_token: CancellationToken) -> anyhow::Result<()> {
    let server_addr: SocketAddr = format!("127.0.0.1:{}", port.unwrap_or(12345)).parse()?;
    let (server, _server_certs) = make_server_endpoint(server_addr)?;
    let channel = QuinnListener::new(server)?;

    println!("Starting RPC server on {}", server_addr);
    let server = RpcServer::<RpcService, QuinnListener<Request, Response>>::new(channel.clone());
    loop {
        let ctrl_c_token = cancellation_token.clone();
        let server_ctrl_token = cancellation_token.clone();
        tokio::select! {
                biased;
                res = server.accept() => {
                    let (req, chan) = res?.read_first().await?;
                    println!("Check accept request: {:?}", req);
                    let _ = Handler{cancellation_token: server_ctrl_token}.handle_rpc_request(req, chan).await;
                }

                ctrl_c_triggered = check_ctrl_c() => {
                    println!("ctrl_c_triggered {:?}", ctrl_c_triggered);
                    if ctrl_c_triggered {
                        println!("Ctrl-C");
                        ctrl_c_token.cancel();
                        println!("Cancelled by ctrl-C");
                        break;
                    }
                }
                _ = sleep(Duration::from_secs(1)) => {}
            }

        let cancellation_token_cloned = cancellation_token.clone();
        let cancelled = check_cancellation_token(cancellation_token_cloned).await;
        if cancelled {
            println!("Cancelled");
            break;
        }
    }
    Ok(())
}