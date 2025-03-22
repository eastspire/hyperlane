use crate::*;

impl Server {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn host(&mut self, host: &'static str) -> &mut Self {
        self.get_mut_cfg().get_mut().set_host(host);
        self
    }

    #[inline]
    pub fn port(&mut self, port: usize) -> &mut Self {
        self.get_mut_cfg().get_mut().set_port(port);
        self
    }

    #[inline]
    pub fn log_dir(&mut self, log_dir: &'static str) -> &mut Self {
        self.get_mut_cfg().get_mut().set_log_dir(log_dir);
        self.get_mut_tmp()
            .get_mut()
            .get_mut_log()
            .set_path(log_dir.into());
        self
    }

    #[inline]
    pub fn log_size(&mut self, log_size: usize) -> &mut Self {
        self.get_mut_cfg().get_mut().set_log_size(log_size);
        self.get_mut_tmp()
            .get_mut()
            .get_mut_log()
            .set_file_size(log_size);
        self
    }

    #[inline]
    pub fn websocket_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        let buffer_size: usize = if buffer_size == 0 {
            DEFAULT_BUFFER_SIZE
        } else {
            buffer_size
        };
        self.get_mut_cfg()
            .get_mut()
            .set_websocket_buffer_size(buffer_size);
        self
    }

    #[inline]
    pub fn inner_print(&mut self, print: bool) -> &mut Self {
        self.get_mut_cfg().get_mut().set_inner_print(print);
        self
    }

    #[inline]
    pub fn inner_log(&mut self, print: bool) -> &mut Self {
        self.get_mut_cfg().get_mut().set_inner_log(print);
        self
    }

    #[inline]
    pub fn enable_inner_print(&mut self) -> &mut Self {
        self.inner_print(true);
        self
    }

    #[inline]
    pub fn disable_inner_print(&mut self) -> &mut Self {
        self.inner_print(false);
        self
    }

    #[inline]
    pub fn enable_inner_log(&mut self) -> &mut Self {
        self.inner_log(true);
        self
    }

    #[inline]
    pub fn disable_inner_log(&mut self) -> &mut Self {
        self.inner_log(false);
        self
    }

    #[inline]
    pub fn log_interval_millis(&mut self, interval_millis: usize) -> &mut Self {
        self.get_mut_cfg()
            .get_mut()
            .set_interval_millis(interval_millis);
        self.get_mut_tmp()
            .get_mut()
            .get_mut_log()
            .set_interval_millis(interval_millis);
        self
    }

    #[inline]
    pub fn route<F, Fut>(&mut self, route: &'static str, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut_route_func: &ArcDashMapRouteFuncBox = self.get_route_func();
        mut_route_func.insert(
            route,
            Box::new(move |controller_data: ControllerData| Box::pin(func(controller_data))),
        );
        self
    }

    #[inline]
    pub fn request_middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_mut_request_middleware().get_mut().push(Box::new(
            move |controller_data: ControllerData| Box::pin(func(controller_data)),
        ));
        self
    }

    #[inline]
    pub fn response_middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.get_mut_response_middleware().get_mut().push(Box::new(
            move |controller_data: ControllerData| Box::pin(func(controller_data)),
        ));
        self
    }

    #[inline]
    async fn get_request_obj_result(
        stream_arc: &ArcRwLockStream,
        websocket_handshake_finish: bool,
        websocket_buffer_size: usize,
    ) -> RequestNewResult {
        if websocket_handshake_finish {
            Request::websocket_request_from_stream(&stream_arc, websocket_buffer_size).await
        } else {
            Request::http_request_from_stream(&stream_arc).await
        }
    }

    #[inline]
    pub async fn listen(&mut self) -> &mut Self {
        {
            self.init();
            let cfg: ServerConfig = self.get_mut_cfg().take();
            let host: &str = *cfg.get_host();
            let port: usize = *cfg.get_port();
            let websocket_buffer_size: usize = *cfg.get_websocket_buffer_size();
            let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
            let tcp_listener: TcpListener = TcpListener::bind(&addr)
                .await
                .map_err(|err| ServerError::TcpBindError(err.to_string()))
                .unwrap();
            while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
                let tmp: Tmp = self.get_mut_tmp().borrow().clone();
                let stream_arc: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
                let mut request_middleware_opt: VecBoxFunc =
                    self.get_mut_request_middleware().take();
                let mut response_middleware_opt: VecBoxFunc =
                    self.get_mut_response_middleware().take();
                let route_func: ArcDashMapRouteFuncBox = self.get_route_func().clone();
                let handle_request = move || async move {
                    let log: Log = tmp.get_log().clone();
                    let mut enable_websocket_opt: Option<bool> = None;
                    let mut websocket_handshake_finish: bool = false;
                    let mut history_request: Request = Request::default();
                    loop {
                        let mut inner_controller_data: InnerControllerData =
                            InnerControllerData::default();
                        let request_obj_result: Result<Request, ServerError> =
                            Self::get_request_obj_result(
                                &stream_arc,
                                websocket_handshake_finish,
                                websocket_buffer_size,
                            )
                            .await
                            .map_err(|err| ServerError::InvalidHttpRequest(err));
                        let init_enable_websocket_opt: bool = enable_websocket_opt.is_some();
                        if request_obj_result.is_err() && !init_enable_websocket_opt {
                            let _ = inner_controller_data
                                .get_mut_response()
                                .close(&stream_arc)
                                .await;
                            return;
                        }
                        let mut request_obj: Request = request_obj_result.unwrap_or_default();
                        if websocket_handshake_finish {
                            history_request.set_body(request_obj.get_body().clone());
                            request_obj = history_request.clone();
                        } else if !init_enable_websocket_opt {
                            history_request = request_obj.clone();
                        }
                        let route: String = request_obj.get_path().clone();
                        inner_controller_data
                            .set_stream(Some(stream_arc.clone()))
                            .set_request(request_obj)
                            .set_log(log.clone());
                        let controller_data: ControllerData =
                            ControllerData::from_controller_data(inner_controller_data);
                        if !init_enable_websocket_opt {
                            enable_websocket_opt =
                                Some(controller_data.judge_enable_websocket().await);
                        }
                        let enable_websocket: bool = enable_websocket_opt.unwrap_or_default();
                        if enable_websocket {
                            let handle_res: ResponseResult = controller_data
                                .handle_websocket(&mut websocket_handshake_finish)
                                .await;
                            if handle_res.is_err() {
                                let _ = controller_data.close().await;
                                return;
                            }
                        }
                        for request_middleware in request_middleware_opt.iter_mut() {
                            request_middleware(controller_data.clone()).await;
                        }
                        if let Some(ref mut async_func) = route_func.get_mut(route.as_str()) {
                            async_func(controller_data.clone()).await;
                        }
                        for response_middleware in response_middleware_opt.iter_mut() {
                            response_middleware(controller_data.clone()).await;
                        }
                        if controller_data.judge_unenable_keep_alive().await && !enable_websocket {
                            let _ = controller_data.close().await;
                            return;
                        }
                    }
                };
                tokio::spawn(handle_request());
            }
        }
        self
    }

    #[inline]
    fn init_log(&mut self) {
        let tmp: Tmp = self.get_mut_tmp().borrow().clone();
        log_run(tmp.get_log());
    }

    #[inline]
    fn init_panic_hook(&mut self) {
        let tmp: Tmp = self.get_mut_tmp().borrow().clone();
        let cfg: ServerConfig = self.get_mut_cfg().borrow().clone();
        let inner_print: bool = *cfg.get_inner_print();
        let inner_log: bool = *cfg.get_inner_log();
        set_hook(Box::new(move |err| {
            let err_msg: String = format!("{}", err);
            if inner_print {
                println_error!(err_msg);
            }
            if inner_log {
                handle_error(&tmp, err_msg.clone());
            }
        }));
    }

    #[inline]
    fn init(&mut self) {
        self.init_panic_hook();
        self.init_log();
    }
}
