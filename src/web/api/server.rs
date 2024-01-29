use std::future::IntoFuture;
use std::io;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use futures::Future;
use listenfd::ListenFd;
use log::info;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot::{self, Sender};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use super::v1::routes::router;
use super::{Running, ServerStartedMessage};
use crate::common::AppData;


/// Starts the API server.
///
/// # Panics
///
/// Panics if the API server can't be started.
pub async fn start(app_data: Arc<AppData>, net_ip: &str, net_port: u16) -> Running {
    let mut listenfd = ListenFd::from_env();
    let tcp_listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            let l = TcpListener::from_std(listener).unwrap();
            let b = l.local_addr().expect("tcp listener to be bound to a socket address.");
            info!("Starting API server with ListenFd: {} ...", b);
            l
        }
        // otherwise fall back to local listening
        None => {
            let config_socket_addr: SocketAddr = match net_ip {
                "localhost" => SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), net_port),
                "*" => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), net_port),
                _ => format!("{net_ip}:{net_port}")
                    .parse()
                    .expect("API server socket address to be valid.")
            };
            info!("Starting API server with net config: {} ...", config_socket_addr);
            TcpListener::bind(config_socket_addr).await.expect("a new server from the previously created tcp listener.")
        }
    };

    let (tx, rx) = oneshot::channel::<ServerStartedMessage>();

    // Run the API server
    let join_handle = tokio::spawn(async move {
        let handle = start_server(tcp_listener, app_data.clone(), tx);

        if let Ok(()) = handle.await {
            info!("API server stopped");
        }

        Ok(())
    });

    // Wait until the API server is running
    let bound_addr = match rx.await {
        Ok(msg) => msg.socket_addr,
        Err(e) => panic!("API server start. The API server was dropped: {e}"),
    };

    Running {
        socket_addr: bound_addr,
        api_server: Some(join_handle),
    }
}

async fn start_server(
    tcp_listener: TcpListener,
    app_data: Arc<AppData>,
    tx: Sender<ServerStartedMessage>,
) -> io::Result<()> {
    let bound_addr = tcp_listener
        .local_addr()
        .expect("tcp listener to be bound to a socket address.");

    info!("API server listening on http://{}", bound_addr);

    let app = router(app_data).layer((
        TraceLayer::new_for_http(),
        // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
        // requests don't hang forever.
        TimeoutLayer::new(Duration::from_secs(10)),
    ));

    tx.send(ServerStartedMessage { socket_addr: bound_addr })
        .expect("the API server should not be dropped");
    axum::serve(tcp_listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal(bound_addr))
        .await
}

async fn shutdown_signal(bound_addr: SocketAddr) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("Stopping API server on http://{} ...", bound_addr);
}