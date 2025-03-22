use crate::*;

impl Server {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub async fn host(&mut self, host: &'static str) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_host(host);
        }
        self
    }

    #[inline]
    pub async fn port(&mut self, port: usize) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_port(port);
        }
        self
    }

    #[inline]
    pub async fn log_dir(&mut self, log_dir: &'static str) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_log_dir(log_dir);
            let mut tmp: RwLockWriteGuard<'_, Tmp> = self.get_tmp().write().await;
            tmp.get_mut_log().set_path(log_dir.into());
        }
        self
    }

    #[inline]
    pub async fn log_size(&mut self, log_size: usize) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_log_size(log_size);
            let mut tmp: RwLockWriteGuard<'_, Tmp> = self.get_tmp().write().await;
            tmp.get_mut_log().set_file_size(log_size);
        }
        self
    }

    #[inline]
    pub async fn websocket_buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        {
            let buffer_size: usize = if buffer_size == 0 {
                DEFAULT_BUFFER_SIZE
            } else {
                buffer_size
            };
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_websocket_buffer_size(buffer_size);
        }
        self
    }

    #[inline]
    pub async fn inner_print(&mut self, print: bool) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_inner_print(print);
        }
        self
    }

    #[inline]
    pub async fn inner_log(&mut self, print: bool) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_inner_log(print);
        }
        self
    }

    #[inline]
    pub async fn enable_inner_print(&mut self) -> &mut Self {
        self.inner_print(true).await;
        self
    }

    #[inline]
    pub async fn disable_inner_print(&mut self) -> &mut Self {
        self.inner_print(false).await;
        self
    }

    #[inline]
    pub async fn enable_inner_log(&mut self) -> &mut Self {
        self.inner_log(true).await;
        self
    }

    #[inline]
    pub async fn disable_inner_log(&mut self) -> &mut Self {
        self.inner_log(false).await;
        self
    }

    #[inline]
    pub async fn log_interval_millis(&mut self, interval_millis: usize) -> &mut Self {
        {
            let mut cfg: RwLockWriteGuard<'_, ServerConfig<'_>> = self.get_cfg().write().await;
            cfg.set_interval_millis(interval_millis);
            let mut tmp: RwLockWriteGuard<'_, Tmp> = self.get_tmp().write().await;
            tmp.get_mut_log().set_interval_millis(interval_millis);
        }
        self
    }

    #[inline]
    pub async fn route<F, Fut>(&mut self, route: &'static str, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        {
            let mut mut_route_func: RwLockWriteGuard<'_, HashMap<&str, Box<dyn Func + Send>>> =
                self.get_route_func().write().await;
            mut_route_func.insert(
                route,
                Box::new(move |controller_data| Box::pin(func(controller_data))),
            );
        }
        self
    }

    #[inline]
    pub async fn request_middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        {
            let mut mut_async_middleware: RwLockWriteGuard<'_, Vec<Box<dyn Func + Send>>> =
                self.get_request_middleware().write().await;
            mut_async_middleware.push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        }
        self
    }

    #[inline]
    pub async fn response_middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: FuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + 'static,
    {
        {
            let mut mut_async_middleware: RwLockWriteGuard<'_, Vec<Box<dyn Func + Send>>> =
                self.get_response_middleware().write().await;
            mut_async_middleware.push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        }
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
            self.init().await;
            let cfg: ServerConfig<'_> = self.get_cfg().read().await.clone();
            let host: &str = *cfg.get_host();
            let port: usize = *cfg.get_port();
            let websocket_buffer_size: usize = *cfg.get_websocket_buffer_size();
            let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
            let tcp_listener: TcpListener = TcpListener::bind(&addr)
                .await
                .map_err(|err| ServerError::TcpBindError(err.to_string()))
                .unwrap();
            while let Ok((stream, _socket_addr)) = tcp_listener.accept().await {
                let tmp_arc_lock: ArcRwLockTmp = self.get_tmp().clone();
                let stream_arc: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
                let async_request_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                    self.get_request_middleware().clone();
                let async_response_middleware_arc_lock: ArcRwLockMiddlewareFuncBox =
                    self.get_response_middleware().clone();
                let route_func_arc_lock: ArcRwLockHashMapRouteFuncBox =
                    self.get_route_func().clone();
                let handle_request = move || async move {
                    let log: Log = tmp_arc_lock.read().await.get_log().clone();
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
                        for request_middleware in
                            async_request_middleware_arc_lock.read().await.iter()
                        {
                            request_middleware(controller_data.clone()).await;
                        }
                        if let Some(async_func) =
                            route_func_arc_lock.read().await.get(route.as_str())
                        {
                            async_func(controller_data.clone()).await;
                        }
                        for response_middleware in
                            async_response_middleware_arc_lock.read().await.iter()
                        {
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
    async fn init_log(&self) {
        let tmp: Tmp = self.get_tmp().read().await.clone();
        log_run(tmp.get_log());
    }

    #[inline]
    async fn init_panic_hook(&self) {
        let tmp: Tmp = self.get_tmp().read().await.clone();
        let cfg: ServerConfig<'_> = self.get_cfg().read().await.clone();
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
    async fn init(&self) {
        self.init_panic_hook().await;
        self.init_log().await;
    }
}
