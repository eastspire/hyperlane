use crate::*;

#[derive(Clone, Setter, Getter)]
pub(crate) struct ServerConfig<'a> {
    pub(super) host: &'a str,
    pub(super) port: usize,
    pub(super) websocket_buffer_size: usize,
    pub(super) http_line_buffer_size: usize,
    pub(super) nodelay: bool,
    pub(super) linger: OptionDuration,
    pub(super) ttl: OptionU32,
    #[set(skip)]
    pub(super) disable_inner_http_handle: ArcRwLock<HashSetXxHash3_64<String>>,
    #[set(skip)]
    pub(super) disable_inner_websocket_handle: ArcRwLock<HashSetXxHash3_64<String>>,
    #[set(skip)]
    pub(super) route_matcher: ArcRwLockRouteMatcher,
    pub(super) error_handle: ArcErrorHandle,
}
