use super::*;

#[derive(Clone, Debug, Lombok)]
pub struct ServerConfig<'a> {
    pub(super) host: &'a str,
    pub(super) port: usize,
    pub(super) log_dir: &'a str,
    pub(super) log_size: usize,
    pub(super) inner_print: bool,
    pub(super) inner_log: bool,
    pub(super) websocket_buffer_size: usize,
    pub(super) http_line_buffer_size: usize,
    pub(super) nodelay: bool,
    pub(super) linger: OptionDuration,
    pub(super) ttl: OptionU32,
}
