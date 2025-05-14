use super::*;

impl Context {
    pub fn from_inner_context(ctx: InnerContext) -> Self {
        Self(arc_rwlock(ctx))
    }

    pub fn from_stream_request_log(stream: &ArcRwLockStream, request: &Request, log: &Log) -> Self {
        let mut inner_ctx: InnerContext = InnerContext::default();
        inner_ctx
            .set_stream(Some(stream.clone()))
            .set_request(request.clone())
            .set_log(log.clone());
        let ctx: Context = Context::from_inner_context(inner_ctx);
        ctx
    }

    pub async fn get(&self) -> InnerContext {
        self.get_read_lock().await.clone()
    }

    pub async fn get_read_lock(&self) -> RwLockReadInnerContext {
        self.0.read().await
    }

    pub async fn get_write_lock(&self) -> RwLockWriteInnerContext {
        self.0.write().await
    }

    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        self.get_read_lock().await.get_stream().clone()
    }

    pub async fn get_request(&self) -> Request {
        self.get_read_lock().await.get_request().clone()
    }

    pub async fn get_response(&self) -> Response {
        self.get_read_lock().await.get_response().clone()
    }

    pub async fn get_request_string(&self) -> String {
        self.get_read_lock().await.get_request().get_string()
    }

    pub async fn get_response_string(&self) -> String {
        self.get_read_lock().await.get_response().get_string()
    }

    pub async fn get_log(&self) -> Log {
        self.get_read_lock().await.get_log().clone()
    }

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

    pub async fn get_socket_addr_string(&self) -> OptionString {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    pub async fn get_route_params_lock(&self) -> ArcRwLockRouteParams {
        self.get_read_lock().await.get_route_params().clone()
    }

    pub async fn get_route_params(&self) -> RouteParams {
        self.get_read_lock()
            .await
            .get_route_params()
            .read()
            .await
            .clone()
    }

    pub async fn get_route_param(&self, name: &str) -> Option<String> {
        self.get_read_lock()
            .await
            .get_route_params()
            .read()
            .await
            .get(name)
            .cloned()
    }

    pub async fn set_route_params(&self, params: RouteParams) -> &Self {
        self.get_write_lock()
            .await
            .set_route_params(arc_rwlock(params));
        self
    }

    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    fn inner_is_websocket(&self, ctx: &RwLockWriteInnerContext) -> bool {
        ctx.get_request().get_upgrade_type().is_websocket()
    }

    async fn inner_send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
        handle_websocket: bool,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut ctx: RwLockWriteInnerContext = self.get_write_lock().await;
            if !handle_websocket && self.inner_is_websocket(&ctx) {
                return Err(ResponseError::NotSupportUseThisMethod);
            }
            let body: ResponseBody = response_body.into();
            let response_res: ResponseResult = ctx
                .get_mut_response()
                .set_body(body)
                .set_status_code(status_code)
                .send(&stream_lock)
                .await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn send_response<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        self.inner_send_response(status_code, response_body, false)
            .await
    }

    pub async fn send(&self) -> ResponseResult {
        let status_code: ResponseStatusCode = self.get_response_status_code().await;
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_response(status_code, response_body).await
    }

    pub async fn send_response_once<T: Into<ResponseBody>>(
        &self,
        status_code: usize,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let mut ctx: RwLockWriteInnerContext = self.get_write_lock().await;
            if self.inner_is_websocket(&ctx) {
                return Err(ResponseError::NotSupportUseThisMethod);
            }
            let response: &mut Response = ctx.get_mut_response();
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

    pub async fn send_once(&self) -> ResponseResult {
        let status_code: ResponseStatusCode = self.get_response_status_code().await;
        let response_body: ResponseBody = self.get_response_body().await;
        self.send_response_once(status_code, response_body).await
    }

    pub async fn send_response_body<T: Into<ResponseBody>>(
        &self,
        response_body: T,
    ) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            let is_websocket: bool = self.get_request_upgrade_type().await.is_websocket();
            let response_res: ResponseResult = self
                .get_write_lock()
                .await
                .get_mut_response()
                .set_body(response_body)
                .send_body(&stream_lock, is_websocket)
                .await;
            return response_res;
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn send_body(&self) -> ResponseResult {
        let body: ResponseBody = self.get_response_body().await;
        self.send_response_body(body).await
    }

    pub async fn close(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            return self
                .get_write_lock()
                .await
                .get_mut_response()
                .close(&stream_lock)
                .await;
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream_lock) = self.get_stream().await {
            return self
                .get_write_lock()
                .await
                .get_mut_response()
                .flush(&stream_lock)
                .await;
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn set_log(&self, log: Log) -> &Self {
        self.get_write_lock().await.set_log(log);
        self
    }

    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.get_read_lock().await.get_log().info(data, func);
        self
    }

    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.get_read_lock().await.get_log().debug(data, func);
        self
    }

    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.get_read_lock().await.get_log().error(data, func);
        self
    }

    pub async fn async_log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.get_read_lock()
            .await
            .get_log()
            .async_info(data, func)
            .await;
        self
    }

    pub async fn async_log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.get_read_lock()
            .await
            .get_log()
            .async_debug(data, func)
            .await;
        self
    }

    pub async fn async_log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: ToString,
        L: LogFuncTrait<T>,
    {
        self.get_read_lock()
            .await
            .get_log()
            .async_error(data, func)
            .await;
        self
    }

    pub async fn get_request_method(&self) -> RequestMethod {
        self.get_read_lock()
            .await
            .get_request()
            .get_method()
            .clone()
    }

    pub async fn get_request_host(&self) -> RequestHost {
        self.get_read_lock().await.get_request().get_host().clone()
    }

    pub async fn get_request_path(&self) -> RequestPath {
        self.get_read_lock().await.get_request().get_path().clone()
    }

    pub async fn get_request_querys(&self) -> RequestQuerys {
        self.get_read_lock()
            .await
            .get_request()
            .get_querys()
            .clone()
    }

    pub async fn get_request_query<T: Into<RequestHeadersKey>>(
        &self,
        key: T,
    ) -> OptionRequestQuerysValue {
        self.get_read_lock()
            .await
            .get_request()
            .get_querys()
            .get(&key.into())
            .map(|data| data.clone())
    }

    pub async fn get_request_body(&self) -> RequestBody {
        self.get_read_lock().await.get_request().get_body().clone()
    }

    pub async fn get_request_body_string(&self) -> String {
        self.get_read_lock().await.get_request().get_body_string()
    }

    pub async fn get_request_body_json<T>(&self) -> ResultSerdeJsonError<T>
    where
        T: DeserializeOwned,
    {
        self.get_read_lock().await.get_request().get_body_json()
    }

    pub async fn get_request_header<K>(&self, key: K) -> OptionRequestHeadersValue
    where
        K: Into<RequestHeadersKey>,
    {
        self.get_read_lock().await.get_request().get_header(key)
    }

    pub async fn get_request_headers(&self) -> RequestHeaders {
        self.get_read_lock()
            .await
            .get_request()
            .get_headers()
            .clone()
    }

    pub async fn get_request_upgrade_type(&self) -> UpgradeType {
        self.get_read_lock()
            .await
            .get_request()
            .get_upgrade_type()
            .clone()
    }

    pub async fn set_request(&self, request_data: Request) -> &Self {
        self.get_write_lock().await.set_request(request_data);
        self
    }

    pub async fn set_request_method<T>(&self, method: T) -> &Self
    where
        T: Into<RequestMethod>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_method(method);
        self
    }

    pub async fn set_request_host<T>(&self, host: T) -> &Self
    where
        T: Into<RequestHost>,
    {
        self.get_write_lock().await.get_mut_request().set_host(host);
        self
    }

    pub async fn set_request_path<T>(&self, path: T) -> &Self
    where
        T: Into<RequestPath>,
    {
        self.get_write_lock().await.get_mut_request().set_path(path);
        self
    }

    pub async fn set_request_query<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<RequestQuerysKey>,
        V: Into<RequestQuerysValue>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_query(key, value);
        self
    }

    pub async fn set_request_querys<T>(&self, querys: T) -> &Self
    where
        T: Into<RequestQuerys>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_querys(querys.into());
        self
    }

    pub async fn set_request_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_header(key, value);
        self
    }

    pub async fn set_request_headers(&self, headers: RequestHeaders) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_request()
            .set_headers(headers);
        self
    }

    pub async fn set_request_body<T: Into<RequestBody>>(&self, body: T) -> &Self {
        self.get_write_lock().await.get_mut_request().set_body(body);
        self
    }

    pub async fn get_response_headers(&self) -> ResponseHeaders {
        self.get_read_lock()
            .await
            .get_response()
            .get_headers()
            .clone()
    }

    pub async fn get_response_header<K>(&self, key: K) -> OptionResponseHeadersValue
    where
        K: Into<ResponseHeadersKey>,
    {
        self.get_read_lock().await.get_response().get_header(key)
    }

    pub async fn get_response_body(&self) -> ResponseBody {
        self.get_read_lock().await.get_response().get_body().clone()
    }

    pub async fn get_response_body_string(&self) -> String {
        self.get_read_lock().await.get_response().get_body_string()
    }

    pub async fn get_response_body_json<T>(&self) -> ResultSerdeJsonError<T>
    where
        T: DeserializeOwned,
    {
        self.get_read_lock().await.get_response().get_body_json()
    }

    pub async fn get_response_reason_phrase(&self) -> ResponseReasonPhrase {
        self.get_read_lock()
            .await
            .get_response()
            .get_reason_phrase()
            .clone()
    }

    pub async fn get_response_status_code(&self) -> ResponseStatusCode {
        self.get_read_lock()
            .await
            .get_response()
            .get_status_code()
            .clone()
    }

    pub async fn set_response(&self, response: Response) -> &Self {
        self.get_write_lock().await.set_response(response);
        self
    }

    pub async fn set_response_header<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_header(key, value);
        self
    }

    pub async fn set_response_headers(&self, headers: ResponseHeaders) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_headers(headers);
        self
    }

    pub async fn set_response_body<T: Into<ResponseBody>>(&self, body: T) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_body(body);
        self
    }

    pub async fn set_response_reason_phrase<T: Into<ResponseReasonPhrase>>(
        &self,
        reason_phrase: T,
    ) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_reason_phrase(reason_phrase);
        self
    }

    pub async fn set_response_status_code(&self, status_code: ResponseStatusCode) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_status_code(status_code);
        self
    }

    pub async fn is_enable_keep_alive(&self) -> bool {
        let ctx: RwLockReadInnerContext = self.get_read_lock().await;
        let headers: &RequestHeaders = ctx.get_request().get_headers();
        if let Some(enable_keep_alive) = headers.iter().find_map(|(key, value)| {
            if key == CONNECTION {
                let value_lowercase: String = value.to_ascii_lowercase();
                if value_lowercase == CONNECTION_KEEP_ALIVE {
                    return Some(true);
                } else if value_lowercase == CONNECTION_CLOSE {
                    return Some(false);
                }
            }
            return None;
        }) {
            return enable_keep_alive;
        }
        let enable_keep_alive: bool = ctx.get_request().get_version().is_http1_1_or_higher();
        return enable_keep_alive;
    }

    pub async fn is_disable_keep_alive(&self) -> bool {
        !self.is_enable_keep_alive().await
    }

    pub async fn is_enable_websocket(&self) -> bool {
        self.get_read_lock()
            .await
            .get_request()
            .get_upgrade_type()
            .is_websocket()
    }

    pub async fn is_disable_websocket(&self) -> bool {
        !self.is_enable_websocket().await
    }

    pub async fn handle_websocket(&self) -> ResponseResult {
        let key_opt: OptionString = self.get_request_header(SEC_WEBSOCKET_KEY).await;
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
                .await;
        }
        Err(ResponseError::WebSocketHandShakeError)
    }

    pub fn format_host_port(host: &str, port: &usize) -> String {
        format!("{}{}{}", host, COLON_SPACE_SYMBOL, port)
    }

    pub async fn set_attribute<T: AnySendSyncClone>(&self, key: &str, value: &T) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_attribute()
            .insert(key.to_owned(), Arc::new(value.clone()));
        self
    }

    pub async fn get_attribute<T: AnySendSyncClone>(&self, key: &str) -> Option<T> {
        self.get_read_lock()
            .await
            .get_attribute()
            .get(key)
            .and_then(|arc| arc.downcast_ref::<T>())
            .cloned()
    }

    pub async fn remove_attribute(&self, key: &str) -> &Self {
        self.get_write_lock().await.get_mut_attribute().remove(key);
        self
    }

    pub async fn clear_attribute(&self) -> &Self {
        self.get_write_lock().await.get_mut_attribute().clear();
        self
    }

    pub async fn get_aborted(&self) -> bool {
        *self.get_write_lock().await.get_aborted()
    }

    pub async fn set_aborted(&self, aborted: bool) -> &Self {
        self.get_write_lock().await.set_aborted(aborted);
        self
    }

    pub async fn aborted(&self) -> &Self {
        self.set_aborted(true).await;
        self
    }

    pub async fn cancel_aborted(&self) -> &Self {
        self.set_aborted(false).await;
        self
    }
}
