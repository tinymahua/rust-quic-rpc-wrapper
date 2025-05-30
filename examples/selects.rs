use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let cancellation_token = CancellationToken::new();
    let service_token = cancellation_token.clone();
    tokio::spawn(async move {
        start_listen(Some(service_token)).await.expect("Failed to start service");
    });

    sleep(Duration::from_secs(10)).await;
    cancellation_token.cancel();

    sleep(Duration::from_secs(10)).await;

    // tokio::spawn(async move {
    //     start_listen(None).await.expect("Failed to start service");
    // });
    // sleep(std::time::Duration::from_secs(20)).await;

    Ok(())
}

// async fn start_listen(cancellation_token: CancellationToken) -> anyhow::Result<()> {
//     let listener = TcpListener::bind("localhost:12345").await?;
//     loop {
//         let cancellation_token_cloned = cancellation_token.clone();
//         tokio::select! {
//             res = listener.accept() => {
//                 match res {
//                     Ok((socket, _)) => {},
//                     Err(err) => {}
//                 }
//             }
//             _ = cancellation_token_cloned.cancelled() => {
//                 break;
//             }
//         }
//     }
//     Ok(())
// }

// async fn start_listen() -> anyhow::Result<()> {
//     let listener = TcpListener::bind("localhost:12345").await?;
//     loop {
//         tokio::select! {
//             res = listener.accept() => {
//                 match res {
//                     Ok((socket, _)) => {},
//                     Err(err) => {}
//                 }
//             }
//             ctrl_c_triggered = tokio::signal::ctrl_c() => {
//                 println!("Ctrl-C triggered");
//                 break;
//             }
//         }
//     }
//     Ok(())
// }


async fn check_cancellation_token(cancellation_token: Option<CancellationToken>) -> bool {
    match cancellation_token {
        Some(token) => {
            token.cancelled().await;
            true
        },
        None => false
    }
}

async fn check_ctrl_c() -> bool {
    tokio::signal::ctrl_c().await.is_ok()
}

async fn start_listen(cancellation_token: Option<CancellationToken>) -> anyhow::Result<()> {
    let listener = TcpListener::bind("localhost:12345").await?;
    loop {
        tokio::select! {
            res = listener.accept() => {
                match res {
                    Ok((socket, _)) => {},
                    Err(err) => {}
                }
            }
            cancelled = check_cancellation_token(cancellation_token.clone()) => {
                println!("Check cancellation token");
                if cancelled {
                    println!("Cancelled");
                    break;
                }
            }
            ctrl_c_triggered = tokio::signal::ctrl_c() => {
                println!("Ctrl-C triggered");
                break;
            }
        }
    }
    Ok(())
}


