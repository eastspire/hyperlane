use crate::*;

#[tokio::test]
async fn test_server() {
    use crate::*;

    async fn request_middleware(ctx: Context) {
        let socket_addr: String = ctx.get_socket_addr_or_default_string().await;
        ctx.set_response_header(SERVER, HYPERLANE)
            .await
            .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
            .await
            .set_response_header(CONTENT_TYPE, TEXT_PLAIN)
            .await
            .set_response_header("SocketAddr", socket_addr)
            .await;
    }

    async fn response_middleware(ctx: Context) {
        let _ = ctx.send().await;
    }

    async fn root_route(ctx: Context) {
        ctx.set_response_status_code(200)
            .await
            .set_response_body("Hello hyperlane => /")
            .await;
    }

    async fn websocket_route(ctx: Context) {
        let request_body: Vec<u8> = ctx.get_request_body().await;
        let _ = ctx.send_response_body(request_body).await;
    }

    fn error_handle(error: String) {
        eprintln!("{}", error);
        let _ = std::io::Write::flush(&mut std::io::stderr());
    }

    async fn main() {
        let server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.enable_nodelay().await;
        server.disable_linger().await;
        server.http_line_buffer_size(4096).await;
        server.websocket_buffer_size(4096).await;
        server.error_handle(error_handle).await;
        server.request_middleware(request_middleware).await;
        server.response_middleware(response_middleware).await;
        server.route("/", root_route).await;
        server.route("/websocket", websocket_route).await;
        server
            .route("/test/:text", move |ctx: Context| async move {
                let param: RouteParams = ctx.get_route_params().await;
                panic!("Test panic {:?}", param);
            })
            .await;
        server.run().await.unwrap();
    }

    let _ = tokio::time::timeout(std::time::Duration::from_secs(60), main()).await;
}
