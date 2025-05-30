use std::time::Duration;
use async_stream::stream;
use derive_more::{From};
use futures_lite::Stream;
use quic_rpc::server::{ChannelTypes, RpcChannel, RpcServerError};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use crate::proto::{ArticleResponse, ListArticlesRequest, Request, RpcService, ShutdownRequest, ShutdownResponse, VersionRequest, VersionResponse};

#[derive(Clone)]
pub struct Handler {
    pub cancellation_token: CancellationToken,
}

impl Handler {
    pub async fn handle_rpc_request<C>(self, msg: Request, chan: RpcChannel<RpcService, C>) -> Result<(), RpcServerError<C>>
    where
        C: ChannelTypes<RpcService>
    {
        println!("Handling RPC request: {:?}", msg);
        match msg {
            Request::Version(ver)=> chan.rpc(ver, self, Self::get_version).await,
            Request::ListArticles(req) => chan.server_streaming(req, self, Self::list_articles).await,
            Request::Shutdown(req) => chan.rpc(req, self, Self::shutdown).await,
        }
    }

    pub async fn get_version(self, _msg:  VersionRequest) -> VersionResponse{
        VersionResponse("0.0.1".to_string())
    }

    pub fn list_articles(self, _msg: ListArticlesRequest) -> impl Stream<Item = ArticleResponse> + Send + 'static {
        let mut articles = vec![
            String::from("Article 1"),
            String::from("Article 2"),
            String::from("Article 3"),
        ];
        let articles_len = articles.len();
        stream! {
                for i in 0..articles_len {
                    yield ArticleResponse {
                        title:  articles[i].clone(),
                    }
                }
            }
    }

    pub async fn shutdown(self, _msg: ShutdownRequest) -> ShutdownResponse {
        let _token = self.cancellation_token.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(3)).await;
            _token.cancel();
        });
        ShutdownResponse
    }
}