use crate::*;

#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn request_middleware(controller_data: ControllerData) {
        let socket_addr: String = controller_data.get_socket_addr_or_default_string().await;
        controller_data
            .set_response_header(SERVER, HYPERLANE)
            .await
            .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
            .await
            .set_response_header(CONTENT_TYPE, content_type_charset(TEXT_PLAIN, UTF8))
            .await
            .set_response_header(DATE, current_date_gmt())
            .await
            .set_response_header("SocketAddr", socket_addr)
            .await;
    }

    async fn response_middleware(controller_data: ControllerData) {
        let _ = controller_data.send().await;
        let request: String = controller_data.get_request_string().await;
        let response: String = controller_data.get_response_string().await;
        controller_data
            .log_info(request, log_handler)
            .await
            .log_info(response, log_handler)
            .await;
    }

    async fn root_route(controller_data: ControllerData) {
        controller_data
            .set_response_status_code(200)
            .await
            .set_response_body("Hello hyperlane => /")
            .await;
    }

    async fn websocket_route(controller_data: ControllerData) {
        let request_body: Vec<u8> = controller_data.get_request_body().await;
        let _ = controller_data.send_response_body(request_body).await;
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(40000).await;
        server.log_dir("./logs").await;
        server.enable_inner_log().await;
        server.enable_inner_print().await;
        server.log_size(100_024_000).await;
        server.log_interval_millis(1000).await;
        server.websocket_buffer_size(4096).await;
        server.request_middleware(request_middleware).await;
        server.response_middleware(response_middleware).await;
        server.route("/", root_route);
        server.route("/websocket", websocket_route);
        let test_string: String = "Hello hyperlane".to_owned();
        server.route(
            "/test/panic",
            async_func!(test_string, |data| {
                println_success!(test_string);
                println_success!(format!("Using external variables {:?}", data));
                panic!("Test panic");
            }),
        );
        server.listen().await;
    }

    run_server().await;
    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(10));
}
