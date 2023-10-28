use axum::{
    body::{self, Body},
    http::{Request, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router
};
use hyper::upgrade::Upgraded;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tower::{make::Shared, ServiceExt};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn handle_proxy() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_|
                    "violet_http_proxy=trace,tower_http=debug".into()
                )
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_svc = Router::new().route("/", get(|| async { "Hello World!" }));

    let service = tower::service_fn(move |req: Request<Body>| {
        let router_svc = router_svc.clone();
        async move {
            if req.method() == Method::CONNECT {
                init_proxy(req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });

    let addr = SocketAddr::from(
        ([127, 0, 0, 1], 3000)
    );

    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service))
        .await
        .unwrap();

}

async fn init_proxy(req: Request<Body>) -> Result<Response, hyper::Error> {
    tracing::trace!(?req);

    if let Some(host_addr) = req.uri().authority().map(|auth| auth.to_string()) {
        tokio::task::spawn(async move {
            match hyper::upgrade::on(req).await {
                Ok(upgraded) => {
                    if let Err(e) = tunnel(upgraded, host_addr).await {
                        tracing::warn!("server io error: {}", e);
                    };
                }
                Err(e) => tracing::warn!("upgrade error: {}", e),
            }
        });

        Ok(Response::new(body::boxed(body::Empty::new())))
    } else {
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        Ok((
            StatusCode::BAD_REQUEST,
            "CONNECT must be to a socket address",
        )
            .into_response())
    }
}

async fn tunnel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;

    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );

    Ok(())
}
