use derive_more::{From, TryInto};
use quic_rpc::message::{Msg, RpcMsg, ServerStreaming, ServerStreamingMsg};
use quic_rpc::Service;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse(pub String);

impl RpcMsg<RpcService> for VersionRequest {
    type Response = VersionResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListArticlesRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub title: String,
}

impl Msg<RpcService> for ListArticlesRequest {
    type Pattern = ServerStreaming;
}

impl ServerStreamingMsg<RpcService> for ListArticlesRequest {
    type Response = ArticleResponse;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShutdownRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShutdownResponse;

impl RpcMsg<RpcService> for ShutdownRequest {
    type Response = ShutdownResponse;
}



#[derive(Debug, Serialize, Deserialize, TryInto, From)]
pub enum Request {
    Version(VersionRequest),
    ListArticles(ListArticlesRequest),
    Shutdown(ShutdownRequest),
}

#[derive(Debug, Serialize, Deserialize, TryInto, From)]
pub enum Response {
    Version(VersionResponse),
    Article(ArticleResponse),
    Shutdown(ShutdownResponse),
}

#[derive(Debug, Clone)]
pub struct RpcService;

impl Service for RpcService{
    type Req = Request;
    type Res = Response;
}

/// Error type for RPC operations
pub type RpcError = serde_error::Error;
/// Result type for RPC operations
pub type RpcResult<T> = Result<T, RpcError>;