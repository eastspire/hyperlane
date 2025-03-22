<center>

## hyperlane

<img src="./img/logo.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/ltpp-universe/hyperlane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane/)

[Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

> Hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, and TCP communication, making it ideal for building modern web services. Additionally, it provides support for request and response middleware, WebSocket, and Server-Sent Events (SSE), enabling flexible and efficient real-time communication.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane
```

## Quick start

- [hyperlane-quick-start git](https://github.com/ltpp-universe/hyperlane-quick-start)
- [hyperlane-quick-start docs](https://docs.ltpp.vip/hyperlane/quick-start/)

```sh
git clone https://github.com/ltpp-universe/hyperlane-quick-start.git
```

## Use

```rust
use hyperlane::*;

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
    server.host("0.0.0.0");
    server.port(10000);
    server.log_dir("./logs");
    server.enable_inner_log();
    server.enable_inner_print();
    server.log_size(100_024_000);
    server.log_interval_millis(1000);
    server.websocket_buffer_size(4096);
    server.request_middleware(request_middleware);
    server.response_middleware(response_middleware);
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
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
