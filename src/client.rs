use std::net::SocketAddr;
use quic_rpc::RpcClient;
use quic_rpc::transport::quinn::{make_insecure_client_endpoint, QuinnConnector};
use crate::proto::{Request, Response, RpcService};
use anyhow::Result;

pub async fn make_rpc_client(port: Option<usize>) -> Result<RpcClient<RpcService, QuinnConnector<Response, Request>>>{
    let server_addr: SocketAddr = format!("127.0.0.1:{}", port.unwrap_or(12345)).parse()?;
    let endpoint = make_insecure_client_endpoint("0.0.0.0:0".parse()?)?;
    println!("Connecting to {}", server_addr);
    let conn: QuinnConnector<Response, Request> = QuinnConnector::new(endpoint, server_addr, "localhost".to_string());
    let client: RpcClient<RpcService, QuinnConnector<Response, Request>> = RpcClient::new(conn);

    Ok(client)
}