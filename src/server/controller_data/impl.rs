use crate::*;

impl ControllerData {
    #[inline]
    pub(crate) fn from_controller_data(controller_data: InnerControllerData) -> Self {
        Self(arc_rwlock(controller_data))
    }

    #[inline]
    pub async fn read(&self) -> ReadInnerControllerData {
        self.0.read().await
    }

    #[inline]
    pub async fn write(&self) -> WriteInnerControllerData {
        self.0.write().await
    }

    #[inline]
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        return self.read().await.get_stream().clone();
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        return self.read().await.get_request().clone();
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        return self.read().await.get_response().clone();
    }

    #[inline]
    pub async fn get_request_string(&self) -> String {
        return self.read().await.get_request().get_string();
    }

    #[inline]
    pub async fn get_response_string(&self) -> String {
        return self.read().await.get_response().get_string();
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        return self.read().await.get_log().clone();
    }

    #[inline]
    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        let socket_addr_opt: OptionSocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .ok();
        socket_addr_opt
    }

    #[inline]
    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        let socket_addr: SocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR);
        socket_addr
    }

    #[inline]
    pub async fn get_socket_addr_string(&self) -> Option<String> {
        let socket_addr_string_opt: Option<String> =
            self.get_socket_addr().await.map(|data| data.to_string());
        socket_addr_string_opt
    }

    #[inline]
    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    #[inline]
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        let addr: OptionSocketAddr = self.get_socket_addr().await;
        let socket_host_opt: OptionSocketHost =
            addr.map(|socket_addr: SocketAddr| socket_addr.ip());
        socket_host_opt
    }

    #[inline]
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        let addr: OptionSocketAddr = self.get_socket_addr().await;
        let socket_port_opt: OptionSocketPort =
            addr.map(|socket_addr: SocketAddr| socket_addr.port());
        socket_port_opt
    }

    #[inline]
    fn inner_is_websocket(controller_data: &WriteInnerControllerData) -> bool {
        return controller_data
            .get_request()
            .get_upgrade_type()
            .is_websocket();
    }

    #[inline]
    async fn inner_send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
        handle_websocket: bool,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            if !handle_websocket && Self::inner_is_websocket(&mut controller_data) {
                return Err(ResponseError::NotSupportUseThisMethod);
            }
            let response: &mut Response = controller_data.get_mut_response();
            let body: ResponseBody = response_body.into();
            let response_res: ResponseResult = response
                .set_body(body)
                .set_status_code(status_code)
                .send(&stream_lock)
                .await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        self.inner_send_response(status_code, response_body, false)
            .await
    }

    #[inline]
    pub async fn send(&self) -> ResponseResult {
        let status_code: ResponseStatusCode = self.get_response_status_code().await;
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_response(status_code, response_body).await
    }

    #[inline]
    pub async fn send_response_once<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            if Self::inner_is_websocket(&mut controller_data) {
                return Err(ResponseError::NotSupportUseThisMethod);
            }
            let response: &mut Response = controller_data.get_mut_response();
            let body: ResponseBody = response_body.into();
            let response_res: ResponseResult = response
                .set_body(body)
                .set_status_code(status_code)
                .send(&stream_lock)
                .await;
            let _ = response.close(&stream_lock).await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn send_once(&self) -> ResponseResult {
        let status_code: ResponseStatusCode = self.get_response_status_code().await;
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_response_once(status_code, response_body).await
    }

    #[inline]
    pub async fn send_response_body<T: Into<ResponseBody>>(
        &self,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let is_websocket: bool = self.get_request_upgrade_type().await.is_websocket();
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response_res: ResponseResult = controller_data
                .get_mut_response()
                .set_body(response_body)
                .send_body(&stream_lock, is_websocket)
                .await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn send_body(&self) -> ResponseResult {
        let body: ResponseBody = self.get_response_body().await;
        self.send_response_body(body).await
    }

    #[inline]
    pub async fn close(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response: &mut Response = controller_data.get_mut_response();
            return response.close(&stream_lock).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            return controller_data.get_mut_response().flush(&stream_lock).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    #[inline]
    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.read().await.get_log().info(data, func);
        self
    }

    #[inline]
    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.read().await.get_log().debug(data, func);
        self
    }

    #[inline]
    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.read().await.get_log().error(data, func);
        self
    }

    #[inline]
    pub async fn get_request_method(&self) -> RequestMethod {
        self.read().await.get_request().get_method().clone()
    }

    #[inline]
    pub async fn get_request_host(&self) -> RequestHost {
        self.read().await.get_request().get_host().clone()
    }

    #[inline]
    pub async fn get_request_path(&self) -> RequestPath {
        self.read().await.get_request().get_path().clone()
    }

    #[inline]
    pub async fn get_request_querys(&self) -> RequestQuerys {
        self.read().await.get_request().get_querys().clone()
    }

    #[inline]
    pub async fn get_request_query<T: Into<RequestHeadersKey>>(
        &self,
        key: T,
    ) -> Option<RequestQuerysValue> {
        self.read()
            .await
            .get_request()
            .get_querys()
            .get(&key.into())
            .and_then(|data| Some(data.clone()))
    }

    #[inline]
    pub async fn get_request_body(&self) -> RequestBody {
        self.read().await.get_request().get_body().clone()
    }

    #[inline]
    pub async fn get_request_body_string(&self) -> String {
        String::from_utf8_lossy(self.read().await.get_request().get_body()).to_string()
    }

    #[inline]
    pub async fn get_request_header<K>(&self, key: K) -> Option<RequestHeadersValue>
    where
        K: Into<RequestHeadersKey>,
    {
        self.read().await.get_request().get_header(key)
    }

    #[inline]
    pub async fn get_request_headers(&self) -> RequestHeaders {
        self.read().await.get_request().get_headers().clone()
    }

    #[inline]
    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        self.read().await.get_request().get_upgrade_type().clone()
    }

    #[inline]
    pub async fn set_request(&self, request_data: Request) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            *request = request_data;
        }
        self
    }

    #[inline]
    pub async fn set_request_method<T>(&self, method: T) -> &Self
    where
        T: Into<RequestMethod>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_method(method);
        }
        self
    }

    #[inline]
    pub async fn set_request_host<T>(&self, host: T) -> &Self
    where
        T: Into<RequestHost>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_host(host);
        }
        self
    }

    #[inline]
    pub async fn set_request_path<T>(&self, path: T) -> &Self
    where
        T: Into<RequestPath>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_path(path);
        }
        self
    }

    #[inline]
    pub async fn set_request_query<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<RequestQuerysKey>,
        V: Into<RequestQuerysValue>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_query(key, value);
        }
        self
    }

    #[inline]
    pub async fn set_request_querys<T>(&self, querys: T) -> &Self
    where
        T: Into<RequestQuerys>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_querys(querys.into());
        }
        self
    }

    #[inline]
    pub async fn set_request_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_header(key, value);
        }
        self
    }

    #[inline]
    pub async fn set_request_headers(&self, headers: RequestHeaders) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_headers(headers);
        }
        self
    }

    #[inline]
    pub async fn set_request_body<T: Into<RequestBody>>(&self, body: T) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let request: &mut Request = controller_data.get_mut_request();
            request.set_body(body);
        }
        self
    }

    #[inline]
    pub async fn get_response_headers(&self) -> ResponseHeaders {
        self.read().await.get_response().get_headers().clone()
    }

    #[inline]
    pub async fn get_response_header<K>(&self, key: K) -> Option<ResponseHeadersValue>
    where
        K: Into<ResponseHeadersKey>,
    {
        self.read().await.get_response().get_header(key)
    }

    #[inline]
    pub async fn get_response_body(&self) -> ResponseBody {
        self.read().await.get_response().get_body().clone()
    }

    #[inline]
    pub async fn get_response_body_string(&self) -> String {
        String::from_utf8_lossy(self.read().await.get_response().get_body()).to_string()
    }

    #[inline]
    pub async fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        self.read().await.get_response().get_reason_phrase().clone()
    }

    #[inline]
    pub async fn get_response_status_code(&self) -> ResponseStatusCode {
        self.read().await.get_response().get_status_code().clone()
    }

    #[inline]
    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response: &mut Response = controller_data.get_mut_response();
            response.set_header(key, value);
        }
        self
    }

    #[inline]
    pub async fn set_response_headers(&self, headers: ResponseHeaders) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response: &mut Response = controller_data.get_mut_response();
            response.set_headers(headers);
        }
        self
    }

    #[inline]
    pub async fn set_response_body<T: Into<ResponseBody>>(&self, body: T) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response: &mut Response = controller_data.get_mut_response();
            response.set_body(body);
        }
        self
    }

    #[inline]
    pub async fn set_response_reason_phrase<T: Into<ResponseReasonPhrase>>(
        &self,
        reason_phrase: T,
    ) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response: &mut Response = controller_data.get_mut_response();
            response.set_reason_phrase(reason_phrase);
        }
        self
    }

    #[inline]
    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        {
            let mut controller_data: WriteInnerControllerData = self.write().await;
            let response: &mut Response = controller_data.get_mut_response();
            response.set_status_code(status_code);
        }
        self
    }

    #[inline]
    pub async fn judge_enable_keep_alive(&self) -> bool {
        let controller_data: ReadInnerControllerData = self.read().await;
        let headers: &RequestHeaders = controller_data.get_request().get_headers();
        if let Some(value) = headers.iter().find_map(|(key, value)| {
            if key.eq_ignore_ascii_case(CONNECTION) {
                Some(value)
            } else {
                None
            }
        }) {
            if value.eq_ignore_ascii_case(CONNECTION_KEEP_ALIVE) {
                return true;
            } else if value.eq_ignore_ascii_case(CONNECTION_CLOSE) {
                return false;
            }
        }
        let enable_keep_alive: bool = controller_data
            .get_request()
            .get_version()
            .is_http1_1_or_higher();
        return enable_keep_alive;
    }

    #[inline]
    pub async fn judge_unenable_keep_alive(&self) -> bool {
        !self.judge_enable_keep_alive().await
    }

    #[inline]
    pub async fn judge_enable_websocket(&self) -> bool {
        let controller_data: ReadInnerControllerData = self.read().await;
        let headers: &RequestHeaders = controller_data.get_request().get_headers();
        return headers.iter().any(|(key, value)| {
            key.eq_ignore_ascii_case(UPGRADE) && value.eq_ignore_ascii_case(WEBSOCKET)
        });
    }

    #[inline]
    pub(crate) async fn handle_websocket(&self, is_handshake: &mut bool) -> ResponseResult {
        if *is_handshake {
            return Ok(());
        }
        let key_opt: Option<String> = self.get_request_header(SEC_WEBSOCKET_KEY).await;
        if let Some(key) = key_opt {
            let accept_key: String = WebSocketFrame::generate_accept_key(&key);
            return self
                .set_response_header(UPGRADE, WEBSOCKET)
                .await
                .set_response_header(CONNECTION, UPGRADE)
                .await
                .set_response_header(SEC_WEB_SOCKET_ACCEPT, accept_key)
                .await
                .inner_send_response(101, "", true)
                .await
                .map(|_| {
                    *is_handshake = true;
                });
        }
        Err(ResponseError::WebSocketHandShakeError)
    }
}
